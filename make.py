import re
from collections import namedtuple
import itertools
import ast
import logging

import jinja2



logger = logging.getLogger(__name__)

CMacroDefinition = namedtuple('CMacroDefinition', ['name', 'type', 'value'])

SpecialFuncTest = namedtuple('SpecialFunctionTest', ['func_c', 'func_rust', 'args', 'expected', 'tolerance'])

SPECIAL_FUNCTION_DICT_C_TO_RUST_MAP = dict(
    gsl_sf_taylorcoeff_e='taylor_coefficient',
    gsl_sf_gamma_e='gamma',
    gsl_sf_lngamma_complex_e='ln_gamma_complex',
    gsl_sf_gammastar_e='gamma_star',
    gsl_sf_gammainv_e='gamma_inv',
    gsl_sf_gamma_complex_e='gamma_complex',
    gsl_sf_fact_e='factorial',
    gsl_sf_lnfact_e='ln_factorial',
    gsl_sf_doublefact_e='double_factorial',
    gsl_sf_lndoublefact_e='double_factorial',
    gsl_sf_hzeta_e='hurwitz_zeta',
    gsl_sf_choose_e='choose',
    gsl_sf_lnchoose_e='ln_choose'
)

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

def parse_TEST_SF_line(line):
    try:
        py_line = convert_c_to_python(line)
        mod = ast.parse(py_line)
        expr = mod.body[0]
        call = expr.value
        
        assert call.func.id == 'TEST_SF'

        func_c = call.args[1].id
        func_rust = SPECIAL_FUNCTION_DICT_C_TO_RUST_MAP.get(func_c, None)
        # Remove the result pointer
        args = []
        
        for arg in call.args[2].elts[:-1]:
            arg_code = ast.get_source_segment(py_line, arg)
            args.append(arg_code)

        expected = ast.get_source_segment(py_line, call.args[3])
        tolerance = ast.get_source_segment(py_line, call.args[4])

        return SpecialFuncTest(
            func_c=func_c,
            func_rust=func_rust,
            args=args,
            expected=expected,
            tolerance=tolerance
        )

    except:
        return None

def inject_docs(path, functions_map, fdocs):
    docs_map = dict((fdoc.name, fdoc) for fdoc in fdocs)
    
    with open(path) as f:
        contents = f.read()

    for (gsl_func, rust_func) in functions_map.items():
        if gsl_func.endswith('_e'):
            normalized_gsl_func = gsl_func[:-2]
        else:
            normalized_gsl_func = gsl_func

        marker = "\npub fn {}(".format(rust_func)

        fdoc = docs_map.get(normalized_gsl_func)

        if fdoc and ("\n" + marker) in contents:
            logger.info("{}: added docs for `{}`".format(path, normalized_gsl_func))
            # Convert markdown text into rust docs (with 3 slashes)
            doc = "\n".join("/// " + line for line in fdoc.doc.split("\n"))
            # Split the contents in the right place
            [before, after] = contents.split("\n" + marker)
            # Update the contents
            contents = "".join([before, "\n\n", doc, marker, after])

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

def generate_special_gamma_test():
    lines = open("gsl/specfunc/test_gamma.c").read().split('\n')

    sf_tests = []

    for line in lines:
        sf_test = parse_TEST_SF_line(line)

        if sf_test and sf_test.func_rust:
            sf_tests.append(sf_test)

    sf_test_groups = []
    for key, group in itertools.groupby(sf_tests, lambda test: test.func_rust):
        sf_test_groups.append((key, list(group)))

    args = dict(sf_test_groups=sf_test_groups)

    test_content = render_template(
        'rs_file_templates/special/gamma_test.rs',
        args
    )

    inject_tests('src/special/gamma.rs', test_content)


from bs4 import BeautifulSoup
from markdownify import markdownify

def process_node_contents(node):
    for child_node in node.select('img.math'):
        # Replace an image that represents math
        child_node.insert_before("$`" + child_node['alt'] + "`$")
        child_node.extract()
    
    for child_node in node.select('a'):
        # Replace a link by its children and
        for grand_child_node in reversed(child_node.contents):
            child_node.insert_before(grand_child_node)
        child_node.extract()

    return node.contents
    
FunctionDoc = namedtuple('FunctionDoc', ['name', 'doc'])

def sf_docs_from_html(path):
    with open(path) as f:
        contents = f.read()

    root = BeautifulSoup(contents, features='lxml')

    functions = root.select('dl.c.function')
    
    docs = []

    for function in functions:
        name = next(iter(function.select('dt.sig.sig-object.c')), None)['id']
        if name.endswith('_e'): name = name[:-2]
        if name.startswith('c.'): name = name[2:]

        docstring_html = "".join(str(child) for child in process_node_contents(function.find('dd')))
        docstring_md = markdownify(docstring_html).strip()
        trimmed_docstring_md = re.sub(r'\n(\s*\n)+', '\n', docstring_md)

        function_doc = FunctionDoc(name=name, doc=trimmed_docstring_md)

        docs.append(function_doc)

    return docs



def main_():
    logging.basicConfig(level=logging.INFO)
    
    function_docs = sf_docs_from_html('gsl_manual/specfunc.html')
    inject_docs("src/special/gamma.rs", SPECIAL_FUNCTION_DICT_C_TO_RUST_MAP, function_docs)
    
    generate_machine_rs()
    generate_special_gamma_test()

SpecialFunctionHead = namedtuple('SfHead', [
    'c_func',
    'rust_func',
    'args',
    'has_single_gsl_sf_result',
    'takes_arrays',
    'no_comments_in_args'
])

SpecialFunctionHeadArg = namedtuple('SpecialFunctionHeadArg', ['name', 'type'])

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
    else: return c_type

def decompose_head(head):
    (gsl_name, raw_args) = head

    rust_name = convert_sf_name_to_rust_name(gsl_name)

    comments_in_args = '/*' not in raw_args

    split_args = [arg.strip().split() for arg in raw_args.split(',')]

    final_args = []
    for arg in split_args:
        if len(arg) == 2:
            typ = convert_to_rust_type(arg[0])
            name = convert_arg_name_to_rust(arg[1])
            sf_arg = SpecialFunctionHeadArg(name=name, type=typ)
            final_args.append(sf_arg)
        
        elif len(arg) == 3:
            if arg[1] == '*':
                typ = arg[0] + "*"
                name = convert_arg_name_to_rust(arg[2])
                sf_arg = SpecialFunctionHeadArg(name=name, type=typ)
                final_args.append(sf_arg)

            else:
                typ = convert_to_rust_type((arg[0], arg[1]))
                name = convert_arg_name_to_rust(arg[2])
                sf_arg = SpecialFunctionHeadArg(name=name, type=typ)
                final_args.append(sf_arg)
        
        elif len(arg) == 4:
                typ = convert_to_rust_type((arg[1], arg[2]))
                name = convert_arg_name_to_rust(arg[3])
                sf_arg = SpecialFunctionHeadArg(name=name, type=typ)
                final_args.append(sf_arg)


        else:
            raise Exception("{} is an invalid arg!".format(arg))
        
    

    gsl_sf_results = [True for arg in final_args if arg.type == 'gsl_sf_result*' ]

    has_single_gsl_sf_result = len(gsl_sf_results) == 1

    takes_arrays = any(arg.type == 'double*' or arg.type == 'int*' for arg in final_args)
        
    return SpecialFunctionHead(
                gsl_name,
                rust_name,
                final_args,
                has_single_gsl_sf_result,
                takes_arrays,
                comments_in_args
            )

def function_heads(path):
    with open(path) as f:
        contents = f.read()

    matches = re.findall(r'int\s+(?P<func>\w+)\((?P<args>[^)]+)\)\s*;', contents)
    
    heads = [decompose_head(match) for match in matches]

    # Remove heads with problematic types
    heads = [head for head in heads if not any('gsl_mode_t' in typ for typ in head.args)]

    return heads

def translate_function_heads(input_path, output_path, template_path):
    heads = function_heads(input_path)
    render_template_into_file(template_path, output_path, dict(sf_heads=heads))
    return heads

SF_MODULES = [
    "airy",
    "bessel",
    "clausen",
    "coupling",
    # "coulomb",
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
    # "gamma",
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

def build_sf_templates():
    default = open('rs_file_templates/special/coulomb.rs').read()
    for module in SF_MODULES:
        with open('rs_file_templates/special/{}.rs'.format(module), 'w+') as f:
            new_contents = default.replace('coulomb', module)
            f.write(new_contents)

def main():
    build_sf_templates()

    for module in SF_MODULES:
        translate_function_heads(
            'gsl/specfunc/gsl_sf_{}.h'.format(module),
            'src/special/{}.rs'.format(module),
            'rs_file_templates/special/{}.rs'.format(module)
        )

    render_template_into_file(
        'rs_file_templates/special.rs',
        'src/special.rs',
        dict(modules=SF_MODULES)
    )

    # translate_function_heads(
    #     'gsl/specfunc/gsl_sf_debye.h',
    #     'src/special/debye.rs',
    #     'rs_file_templates/special/debye.rs'
    # )

    # translate_function_heads(
    #     'gsl/specfunc/gsl_sf_coulomb.h',
    #     'src/special/coulomb.rs',
    #     'rs_file_templates/special/coulomb.rs'
    # )