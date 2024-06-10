import re
from collections import namedtuple
import itertools
import ast
import logging
import os
import json

import jinja2



logger = logging.getLogger(__name__)

CMacroDefinition = namedtuple('CMacroDefinition', ['name', 'type', 'value'])

SpecialFuncTest = namedtuple('SpecialFunctionTest', ['func_c', 'func_rust', 'args', 'expected', 'tolerance'])

FunctionDoc = namedtuple('FunctionDoc', ['name', 'doc'])

SpecialFunctionHead = namedtuple('SpecialFunctionHead', [
    'c_func',
    'rust_func',
    'args',
    'has_single_gsl_sf_result',
    'takes_arrays',
    'no_comments_in_args'
])

FunctionHeadArg = namedtuple('FunctionHeadArg', ['name', 'type'])

def inject_tests(path, tests_content):
    with open(path) as f:
        contents = f.read()

    [code, old_tests] = contents.split("\n#[cfg(test)]")
    new_contents = "\n".join([code, tests_content])
    
    with open(path, 'w') as f:
        f.write(new_contents)

def render_template_into_file(template_path, output_path, template_args):
    template = jinja2.Template(open(template_path).read())
    contents = template.render(template_args)
    with open(output_path, 'w') as f:
        f.write(contents)

def render_template(template_path, template_args):
    template = jinja2.Template(open(template_path).read())
    contents = template.render(template_args)
    return contents

def convert_c_to_python(c_line):
    return c_line.replace('&', '__POINTER_REF__').replace(';', '').strip()


def floating_point_value_to_rust(stringified_float):
    # Floating point values can't start with a dot.
    # For example, this is invalid `.123`.
    # We must make sure that floating point values start with a zero (0).

    s = stringified_float.strip()

    if s.startswith('.'):
        s = "0" + s
        
    if s.startswith('-.'):
        s = "-0." + s[2:]

    if s.startswith('+.'):
        s = "+0." + s[2:]
        
    if s.startswith('+'):
        s = s[1:]

    return s


def is_constant_integer(value):
    is_positive_integer = (
        isinstance(value, ast.Constant) and
        isinstance(value.value, int)
    )

    is_negative_integer = (
        isinstance(value, ast.UnaryOp) and
        isinstance(value.op, ast.USub) and
        isinstance(value.operand, ast.Constant) and
        isinstance(value.operand.value, int)
    )

    return is_positive_integer or is_negative_integer


def parse_TEST_SF_line(line, db):
    try:
        py_line = convert_c_to_python(line)
        mod = ast.parse(py_line)
        expr = mod.body[0]
        call = expr.value
        
        assert call.func.id == 'TEST_SF'

        c_func = call.args[1].id
        function = db.functions.get(c_func, None)

        if function and 'args' in function:
            rust_func = function['rust_func']

            # Remove the result pointer
            args = []
            
            for (typed_arg, arg_value) in zip(function['args'], call.args[2].elts[:-1]):
                typ = typed_arg.type
                serialized = ast.get_source_segment(py_line, arg_value)
                
                if is_constant_integer(arg_value) and typ == 'f64':
                    arg_code = str(float(serialized)).lstrip('+')
                else:
                    arg_code = ast.get_source_segment(py_line, arg_value).lstrip('+')

                # Exclude all arguments that contain variables
                # Those tests will have to be translated manually.
                assert not re.match(r'[abcdf-zABCD-Z]', arg_code)

                # Hack for a nasty particular case:
                arg_code = arg_code.replace('-100-1.0/65536.0', '-100.0-1.0/65536.0')

                args.append(arg_code)


            if is_constant_integer(call.args[3]):
                expected = str(float(ast.get_source_segment(py_line, call.args[3])))
            
            else:    
                expected = ast.get_source_segment(py_line, call.args[3])
                # Floating point values can't start with a dot.
                # For example, this is invalid `.123`.
                # We must make sure that floating point values start with a zero (0).
                expected = floating_point_value_to_rust(expected)

            assert not re.match(r'[abcdf-zABCD-Z]', expected)

            tolerance = ast.get_source_segment(py_line, call.args[4])

            return SpecialFuncTest(
                func_c=c_func,
                func_rust=rust_func,
                args=args,
                expected=expected,
                tolerance=tolerance
            )
        
        else:
            return None

    except Exception as e:
        if isinstance(e, (AttributeError, SyntaxError, IndexError, AssertionError)):
            return None
        else:
            raise e

def inject_docs(path, db, fdocs):
    docs_map = dict((fdoc.name, fdoc) for fdoc in fdocs)
    
    with open(path) as f:
        contents = f.read()

    for (gsl_func, function) in db.functions.items():
        rust_func = function['rust_func']
        c_func_canonical = function['c_func_canonical']

        marker_with_warning_suppression = "\n#[allow(non_snake_case)]\npub fn {}(".format(rust_func)

        if marker_with_warning_suppression in contents:
            marker = marker_with_warning_suppression
        else:
            marker = "\npub fn {}(".format(rust_func)

        fdoc = docs_map.get(c_func_canonical)

        link_to_gsl = "\n///\n/// Binds the [`{}`](https://www.gnu.org/software/gsl/doc/html/specfunc.html#c.{}) function.".format(gsl_func, gsl_func)

        if fdoc and ("\n" + marker) in contents:
            # Convert markdown text into rust docs (with 3 slashes)
            doc = "\n".join("/// " + line for line in fdoc.doc.split("\n"))
            # Split the contents in the right place
            [before, after] = contents.split("\n" + marker)
            # Update the contents (adding support for KaTeX!)
            
            contents = "".join([
                before,
                "\n\n#[cfg_attr(doc, katexit::katexit)]\n",
                "/// <style>p { overflow-y: hidden; }</style>\n",
                doc,
                link_to_gsl,
                marker,
                after
            ])

    with open(path, 'w') as f:
        f.write(contents)


def generate_machine_rs():
    lines = open("gsl/gsl_machine.h").read().split("\n")

    cmacros = []

    for line in lines:
        m = re.match(r'#define\s+(?P<var>\w+)\s+\(?(?P<value>[0-9e\-\+.]+)\)?', line)
        if m:
            if m.group('var').startswith('GSL_'):
                renamed_var = m.group('var')[4:]
            else:
                renamed_var = m.group('var')

            value = m.group('value')
            cmacro = CMacroDefinition(name=renamed_var, type='f64', value=value)
            cmacros.append(cmacro)

    args = dict(cmacros=cmacros)

    render_template_into_file('rs_file_templates/machine.rs', 'src/machine.rs', args)

def inject_tests_in_module(module, db):
    test_path_c = "gsl/specfunc/test_{}.c".format(module)

    if os.path.exists(test_path_c):
        lines = open(test_path_c).read().split('\n')

        sf_tests = []

        for line in lines:
            sf_test = parse_TEST_SF_line(line, db)

            if sf_test and sf_test.func_rust:
                sf_tests.append(sf_test)

        sf_tests = sorted(sf_tests, key=lambda test: test.func_rust)

        sf_test_groups = []
        for key, group in itertools.groupby(sf_tests, lambda test: test.func_rust):
            sf_test_groups.append((key, list(group)))

        args = dict(sf_test_groups=sf_test_groups)

        test_content = render_template(
            'rs_file_templates/special/special_functions_test.rs',
            args
        )

        inject_tests('src/special/{}.rs'.format(module), test_content)



from bs4 import BeautifulSoup
from markdownify import markdownify
import latex2mathml.converter
import hashlib

def process_node_contents(node):
    for child_node in node.select('img'):
        # Replace an image that represents math
        child_node.insert_before("$" + child_node['alt'] + "$")
        child_node.extract()
    
    for child_node in node.select('a'):
        # Replace a link by its children and
        for grand_child_node in reversed(child_node.contents):
            child_node.insert_before(grand_child_node)
        child_node.extract()

    return node.contents
    

def canonical_c_func_name(name):
    if name.endswith('_e'): name = name[:-2]
    if name.startswith('c.'): name = name[2:]
    return name

def sf_docs_from_html(path):
    with open(path) as f:
        contents = f.read()

    root = BeautifulSoup(contents, features='lxml')

    functions = root.select('dl.c.function')
    
    docs = []

    for function in functions:
        for sig_object in function.select('dt.sig.sig-object.c'):
            name = sig_object['id']
            if not name.endswith('_e'):
                if name.startswith('c.'): name = name[2:]

                processed_contents = process_node_contents(function.find('dd'))

                docstring_html = "".join([str(child) for child in processed_contents])
                docstring_md = markdownify(docstring_html).strip()

                trimmed_docstring_md = re.sub(r'\n(\s*\n)+', '\n', docstring_md)

                function_doc = FunctionDoc(name=name, doc=trimmed_docstring_md)

                docs.append(function_doc)

    return docs



def convert_sf_name_to_rust_name(c_func):
    if c_func.endswith('_e'): c_func = c_func[:-2]
    if c_func.startswith('gsl_sf_'): c_func = c_func[7:]
    return c_func

def convert_arg_name_to_rust(arg_name):
    return arg_name.lower()

def convert_to_rust_type(c_type):
    if c_type == 'double': return 'f64'
    elif c_type == 'int': return 'i32'
    elif c_type == ('const', 'int'): return 'i32'
    elif c_type == ('unsigned', 'int'): return 'u32'
    elif c_type == ('const', 'double'): return 'f64'
    elif c_type == ('double', '*'): return 'double*'
    elif c_type == ('int', '*'): return 'double*'
    elif c_type == ('const', 'gsl_mode_t'): return 'gsl_mode_t'
    elif c_type is None: return ""
    else: return c_type


def decompose_head(head, db):
    (gsl_name, raw_args) = head

    function = db.functions.get(gsl_name)
    rust_name = function and function.get('rust_func')

    comments_in_args = '/*' not in raw_args

    split_args = [arg.strip().split() for arg in raw_args.split(',')]

    final_args = []
    for arg in split_args:
        if len(arg) == 2:
            typ = convert_to_rust_type(arg[0])
            name = convert_arg_name_to_rust(arg[1])
            sf_arg = FunctionHeadArg(name=name, type=typ)
            final_args.append(sf_arg)
        
        elif len(arg) == 3:
            if arg[1] == '*':
                typ = arg[0] + "*"
                name = convert_arg_name_to_rust(arg[2])
                sf_arg = FunctionHeadArg(name=name, type=typ)
                final_args.append(sf_arg)

            else:
                typ = convert_to_rust_type((arg[0], arg[1]))
                name = convert_arg_name_to_rust(arg[2])
                sf_arg = FunctionHeadArg(name=name, type=typ)
                final_args.append(sf_arg)
        
        elif len(arg) == 4:
                typ = convert_to_rust_type((arg[1], arg[2]))
                name = convert_arg_name_to_rust(arg[3])
                sf_arg = FunctionHeadArg(name=name, type=typ)
                final_args.append(sf_arg)

        else:
            raise Exception("{} is an invalid arg!".format(arg))
        
    
    gsl_sf_results = [True for arg in final_args if arg.type == 'gsl_sf_result*' ]

    has_single_gsl_sf_result = len(gsl_sf_results) == 1

    takes_arrays = any(arg.type == 'double*' or arg.type == 'int*' or arg.type == '*sgn'
                       for arg in final_args)
        
    return SpecialFunctionHead(
                gsl_name,
                rust_name,
                final_args,
                has_single_gsl_sf_result,
                takes_arrays,
                comments_in_args
            )


def c_arg_to_rust(typ):
    if typ == ('const', 'double') or typ == ('double',) or typ == 'double':
        return 'f64'
    elif typ == ('const', 'int') or typ == ('int',) or typ == 'double':
        return 'i32'
    elif typ == ('const', 'unsigned', 'int') or typ == ('unsigned', 'int') or typ == 'unsigned int':
        return 'u32'
    elif typ == ('const', 'gsl_rng', '*'):
        return 'mut Rng'
    elif len(typ) == 1:
        return typ[0]
    elif isinstance(typ, str):
        return typ
    else:
        return " ".join(typ)

def decompose_normal_function_head(match, db):
    (func_typ, gsl_name, raw_args) = match

    split_args = [arg.strip().split() for arg in raw_args.split(',')]

    if gsl_name.startswith('gsl_ran_') and not gsl_name.endswith('_pdf'):
        rust_name = 'random_' + gsl_name[8:]
    else:
        rust_name = gsl_name[8:]

    final_args = []
    for arg in split_args:
        typ = c_arg_to_rust(tuple(arg[:-1]))
        name = arg[-1]
        
        final_arg = FunctionHeadArg(name=name, type=typ)
        final_args.append(final_arg)
                
    return FunctionHead(
                gsl_name,
                rust_name,
                final_args,
                c_arg_to_rust(func_typ)
            )

FunctionHead = namedtuple('FunctionHead', ['name', 'rust_name', 'args', 'type'])

def function_heads(path, db):
    with open(path) as f:
        contents = f.read()

    matches = re.findall(r'int\s+(?P<func>\w+)\s*\((?P<args>[^)]+)\)\s*;', contents)
    
    heads = [decompose_head(match, db) for match in matches]

    # Remove heads with problematic types
    heads = [
        head for head in heads if
        not any('gsl_mode_t' in arg.type or '*' in arg.name for arg in head.args) and
        head.has_single_gsl_sf_result and
        not head.takes_arrays and
        not head.takes_arrays
    ]

    return heads

def normal_function_heads(path, db):
    with open(path) as f:
        contents = f.read()

    matches = re.findall(r'\n(?P<type>(?:unsigned\s+)?\w+)\s+(?P<func>\w+)\s+\((?P<args>[^)]+)\)\s*;', contents)
    
    heads = [decompose_normal_function_head(match, db) for match in matches]

    heads = [
        head for head in heads
        if all(arg.type in ['f64', 'u32', 'u32', 'mut Rng'] for arg in head.args) and
        head.type != 'void'
    ]

    return heads

def translate_function_heads(input_path, output_path, template_path, db):
    heads = function_heads(input_path, db)
    render_template_into_file(template_path, output_path, dict(sf_heads=heads))

    return heads

SF_MODULES = [
    "airy",
    "bessel",
    "clausen",
    "coupling",
    "coulomb",
    "dawson",
    "debye",
    "dilog",
    "elementary",
    "ellint",
    "elljac",
    "erf",
    "exp",
    "expint",
    "fermi_dirac",
    "gamma",
    "gegenbauer",
    "hermite",
    "hyperg",
    "laguerre",
    "lambert",
    "legendre",
    "log",
    "mathieu",
    "pow_int",
    "psi",
    "sincos_pi",
    "synchrotron",
    "transport",
    "trig",
    "zeta"
]

def inject_tests_in_modules(modules, db):
    for module in modules:
        inject_tests_in_module(module, db)

def gather_function_heads(modules, db):
    heads = []
    
    for module in modules:
        new_heads = function_heads('gsl/specfunc/gsl_sf_{}.h'.format(module), db)
        heads.extend(new_heads)

    return heads

def build_sf_templates():
    default = open('rs_file_templates/special/coulomb.rs').read()
    for module in SF_MODULES:
        with open('rs_file_templates/special/{}.rs'.format(module), 'w+') as f:
            new_contents = default.replace('coulomb', module)
            f.write(new_contents)


def translate_function_head_in_modules(modules, db):
    # Translate the function heads into safe rust
    for module in modules:
        translate_function_heads(
            'gsl/specfunc/gsl_sf_{}.h'.format(module),
            'src/special/{}.rs'.format(module),
            'rs_file_templates/special/{}.rs'.format(module),
            db
        )

def inject_docs_in_modules(modules, db):
    # Get function docs from the manual
    function_docs = sf_docs_from_html('gsl_manual/specfunc.html')

    # Add the docs to all modules (if there aren't already some docs there)
    for module in modules:
        inject_docs("src/special/{}.rs".format(module), db, function_docs)

class SpecialFunctionDatabase:

    def __init__(self, path):
        self.path = path
        functions = json.load(open('gsl_functions.json'))
        
        self.functions = dict()
        for func in functions:
            self.functions[func['c_func']] = func

    def update_with_modules(self, modules):
        heads = gather_function_heads(modules, self)

        for head in heads:
            if head.c_func not in self.functions:
                new_func = {
                    'c_func': head.c_func,
                    'c_func_canonical': canonical_c_func_name(head.c_func),
                    'rust_func': head.rust_func,
                    'exclude': False,
                    'args': head.args
                }

                self.functions[head.c_func] = new_func
            
        self.add_typing_information(modules)


    def add_typing_information(self, modules):
        for module in modules:
            path = 'gsl/specfunc/gsl_sf_{}.h'.format(module)
            heads = function_heads(path, self)

            for head in heads:
                self.functions[head.c_func]['args'] = head.args



def build_randist(db):
    path = "gsl/randist/gsl_randist.h"
    function_heads = normal_function_heads(path, db)

    render_template_into_file(
        "rs_file_templates/randist.rs",
        "src/randist.rs",
        dict(function_heads=function_heads)
    )


def main():
    logging.basicConfig(level=logging.INFO)

    db = SpecialFunctionDatabase('gsl_functions.json')
    db.update_with_modules(SF_MODULES)

    # Build the templates that will be used below;
    # If we customize these templates, we must remove
    # those templates from the list!

    build_sf_templates()
    
    render_template_into_file(
        'rs_file_templates/special.rs',
        'src/special.rs',
        dict(modules=SF_MODULES)
    )

    generate_machine_rs()

    translate_function_head_in_modules(SF_MODULES, db)

    inject_docs_in_modules(SF_MODULES, db)

    inject_tests_in_modules(SF_MODULES, db)

    build_randist(db)

if __name__ == "__main__":
    main()