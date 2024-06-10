defmodule RSci do
  @moduledoc """
  Documentation for `RSci`.
  """
  alias RSci.GslFunction

  @public_cacerts :public_key.cacerts_get()

  @manual_html_url "https://www.gnu.org/software/gsl/doc/html/"

  def wget(url) do
    url_charlist = to_charlist(url)

    ssl_options = [
      ssl: [
        verify: :verify_peer,
        cacerts: @public_cacerts
      ]
    ]

    {:ok, resp} = :httpc.request(:get, {url_charlist, []}, ssl_options, [body_format: :binary])
    {{_, 200, 'OK'}, _headers, body} = resp

    body
  end

  def scrape_html_manual() do
    html = wget(@manual_html_url)

    {:ok, document} = html |> Floki.parse_document()

    local_urls = Floki.find(document, "li.toctree-l1 > a.reference.internal") |> Floki.attribute("href")

    for local_url <- local_urls do
      full_url = @manual_html_url <> local_url
      html = wget(full_url)
      File.write!("priv/gsl_manual/" <> local_url, html)
    end
  end

  def with_last(elements) do
    count = Enum.count(elements)
    Enum.with_index(elements, fn element, index -> {element, index == count - 1} end)
  end

  def math_image?(img) do
    case Floki.attribute(img, "src") do
      [src] ->
        String.starts_with?(src, "_images/math/")

      _other ->
        false
    end
  end

  def html_to_md(html) do
    html_to_md_helper(html)
  end

  def html_to_md_helper(html) do
    Floki.traverse_and_update(html, fn
      {"div", [{"class", "math"}], [text, "\n\n"]} when is_binary(text) ->
        ["$#{text}$", "\n\n"]

      {"img", _attrs, _contents} = img ->
        if math_image?(img) do
          [inline_math] = Floki.attribute(img, "alt")
          "$#{inline_math}$"

        else
          [alt] = Floki.attribute(img, "alt")
          [src] = Floki.attribute(img, "src")

          "![#{alt}](https://www.gnu.org/software/gsl/doc/html/#{src})\n\n"
        end

      {"p", _attrs, contents} ->
        contents ++ ["\n\n"]

      {"code", _attrs, contents} ->
        ["`"] ++ contents ++ ["`"]

      {"span", _attrs, contents} ->
        contents

      {_tag, _attrs, contents} ->
        contents

      other ->
        other
    end)
  end

  def html_to_text(html) do
    iolist = Floki.traverse_and_update(html, fn
      {_tag, _attr, contents} ->
        contents

      other ->
        other
    end)

    iolist
    |> List.flatten()
    |> Enum.intersperse(" ")
    |> Enum.join()
    |> String.replace("¶", "")
    |> String.trim()
  end

  def html_doc_to_md(html, module_name, gsl_function_name) do
    md_list = html_to_md(html)

    url = "#{@manual_html_url}/#{module_name}.html#c.#{gsl_function_name}"

    reference_line = "Binds the function [`#{gsl_function_name}`](#{url})."

    _md_doc = IO.iodata_to_binary([md_list, reference_line])
  end

  def c_type_to_rust_type(c_type) do
    c_type =
      case c_type do
        "const " <> rest -> rest
        other -> other
      end

    case c_type do
      "double" -> "f64"
      "float" -> "f32"
      "int" -> "i32"
      "long int" -> "i64"
      "unsigned int" -> "u32"
      "long unsigned int" -> "u64"
      "gsl_rng *" -> "Rng"
      "gsl_sf_result *" -> :gsl_result
      _other -> :unknown
    end
  end

  alias RSci.GslFunctionArg
  alias RSci.RustFunctionArg


  def gsl_module_to_rust_module(module) do
    case module do
      "specfunc" ->
        "special"

      "randist" ->
        "distribution"

      other ->
        other
    end
  end

  def build_c_arguments(raw_c_arguments) do
    for {type, name} <- raw_c_arguments do
      %GslFunctionArg{type: type, name: name}
    end
  end

  def c_argument_name_to_rust_name(name) do
    String.downcase(name)
  end

  def c_arguments_to_rust_arguments(c_arguments) do
    for c_arg <- c_arguments do
      rust_name = c_argument_name_to_rust_name(c_arg.name)
      rust_type = c_type_to_rust_type(c_arg.type)
      %RustFunctionArg{type: rust_type, name: rust_name}
    end
  end

  def is_sf_e_function?(function) do
    String.starts_with?(function.c_name, "gsl_sf_") and String.ends_with?(function.c_name, "_e")
  end

  def transform_sf_e_function(function) do
    if String.starts_with?(function.c_name, "gsl_sf_") and String.ends_with?(function.c_name, "_e") do
      new_return_type = "Result<ValWithError<f64>>"
      new_args = for arg <- function.rust_arguments, arg.type != :gsl_result, do: arg

      %{function | rust_arguments: new_args, rust_return_type: new_return_type}
    else
      function
    end
  end

  def group_sf_functions(functions) do
    Enum.group_by(functions, fn fun ->
      cond do
        is_sf_e_function?(fun) -> :sf_e_functions
        true -> :sf_normal_functions
      end
    end)
  end

  def heuristic_c_func_name_to_rust_name(function_name) do
    case function_name do
      "gsl_ran_chisq" <> rest ->
        "chi2" <> rest

      "gsl_cdf_chisq_" <> rest ->
        "chi2_" <> rest <> "_cdf"

      "gsl_ran_tdist" <> rest ->
        "student_t" <> rest

      "gsl_cdf_tdist_" <> rest ->
        "student_t_" <> rest <> "_cdf"

      "gsl_ran_" <> rest ->
        if (
              String.ends_with?(rest, "_pdf") or
              String.ends_with?(rest, "_cdf") or
              String.ends_with?(rest, "P") or
              String.ends_with?(rest, "Q")
            ) do
          rest
        else
          "#{rest}_rvs"
        end

      "gsl_sf_" <> rest ->
        cond do
          String.contains?(rest, "ln") and rest != "ln" ->
            String.replace(rest, "ln", "ln_")

          String.contains?(rest, "zetam1") ->
            String.replace(rest, "zetam1", "zeta_minus_1")

          true ->
            rest
        end

      "gsl_cdf_" <> rest ->
        rest <> "_cdf"

      "gsl_sf_fact" ->
        "factorial"

      "gsl_sf_doublefact" ->
        "double_factorial"

      "gsl_sf_lnfact" ->
        "ln_factorial"

      "gsl_sf_lndoublefact" ->
        "ln_double_factorial"

      other ->
        other
    end
  end

  def nested_extract_functions_for_gsl_module(gsl_module) do
    path = "priv/gsl_manual/#{gsl_module}.html"

    rust_module = gsl_module_to_rust_module(gsl_module)

    html = File.read!(path)
    {:ok, doc} = Floki.parse_document(html)

    _nested_docs =
      for function_group <- Floki.find(doc, "dl.c.function") do
        function_calls = Floki.find(function_group, "dt")

        case Floki.find(function_group, "dd") do
          [doc] ->
            for function_call <- function_calls do
              ["c." <> function_name] = Floki.attribute(function_call, "id")

              converted_doc = html_doc_to_md(doc, gsl_module, function_name)

              rust_name = heuristic_c_func_name_to_rust_name(function_name)

              call_text = html_to_text(function_call)

              {c_type, _c_function, raw_c_arguments} =
                case RSci.GslFunctionDefParser.parse(call_text) do
                  {:ok, result} ->
                    result

                  _other ->
                    {nil, nil, []}
                end

              c_arguments = build_c_arguments(raw_c_arguments)
              rust_arguments = c_arguments_to_rust_arguments(c_arguments)

              # Special case due to error in the documentation
              rust_type =
                if String.starts_with?(function_name, "gsl_sf_mathieu") do
                  "f64"
                else
                  c_type_to_rust_type(c_type)
                end

              function = %GslFunction{
                c_name: function_name,
                rust_name: rust_name,
                c_module: gsl_module,
                rust_module: rust_module,
                c_arguments: c_arguments,
                rust_arguments: rust_arguments,
                c_return_type: c_type,
                rust_return_type: rust_type,
                doc: converted_doc
              }

              {function_name, function}
            end

          _other ->
            []
        end
      end
  end

  def extract_functions_for_gsl_modules(gsl_modules) do
    nested_functions =
      for gsl_module <- gsl_modules do
        nested_extract_functions_for_gsl_module(gsl_module)
      end

    List.flatten(nested_functions)
    |> Enum.reject(fn {_name, f} -> unknown_types?(f.rust_arguments) end)
    |> Enum.into(%{})
  end

  def extract_functions_for_all_gsl_modules() do
    _all_modules = File.ls!("priv/gsl_manual")
    all_modules = ["specfunc", "randist"]

    gsl_modules =
      for gsl_module_html_file <- all_modules do
        Path.basename(gsl_module_html_file, ".html")
      end

    functions_without_gsl_path = extract_functions_for_gsl_modules(gsl_modules)

    paths =
      for path <- Path.wildcard("../gsl/**/*.h"), not String.starts_with?(path, "../gsl/ampl/") do
        path
      end

    function_name_to_file =
      Enum.reduce(paths, %{}, fn path, acc ->
        rel_path = Path.relative_to(path, "../gsl")
        text = File.read!(path)
        functions = Regex.scan(~r/\n\w+\s+(\w+)\(/, text)
        new_funcs = for [_, func_name] <- functions, do: func_name

        Enum.reduce(new_funcs, acc, fn new_func, new_acc ->
          Map.put(new_acc, new_func, rel_path)
        end)
      end)

    for {key, function} <- functions_without_gsl_path, not black_list?(function), into: %{} do
      path = Map.get(function_name_to_file, function.c_name)
      submodule = path_to_submodule(path)

      new_function = %{function | gsl_path: path, gsl_submodule: submodule}

      {key, new_function}
    end
  end

  def black_list?(function) do
    String.contains?(function.c_name, "airy") or
      String.contains?(function.c_name, "_complex_") or
      function.c_name == "gsl_sf_polar_to_rect" or
      function.c_name == "gsl_sf_rect_to_polar"
  end

  def path_to_submodule(nil), do: nil

  def path_to_submodule(path) do
    case Path.basename(path, ".h") do
      "gsl_sf_" <> rest -> rest
      "gsl_sf_ran" <> rest -> rest
      "gsl_" <> rest -> rest
    end
  end

  def filter_functions(functions, filter) do
    for {_name, function} <- functions, filter.(function) do
      function
    end
  end

  def unknown_types?(args) do
    Enum.any?(args, fn arg -> arg.type == :unknown end)
  end

  def replace_symbols_missing_in_katex(text) do
    text
    |> String.replace("\\sinc", "\\text{sinc}")
    |> String.replace("\\erfc", "\\text{erfc}")
    |> String.replace("\\erf", "\\text{erf}")
  end

  def doc_to_rust(text) do
    lines =
      text
      |> String.replace(".  ", ".\n")
      |> replace_symbols_missing_in_katex()
      |> String.split("\n")
      |> Enum.map(fn line -> ["/// ", line] end)

    all_lines = lines

    Enum.join(all_lines, "\n")
  end

  def traverse_expression(ast, fun) do
    result = fun.(ast)

    case result do
      {:ok, something} ->
        something

      :error ->
        case ast do
          {op, [a, b]} when op in [:+, :-, :*, :/] ->
            {op, [traverse_expression(a, fun), traverse_expression(b, fun)]}

          {:parenthesis, expr} ->
            {:parenthesis, traverse_expression(expr, fun)}

          other ->
            other
        end
    end
  end

  def coerce_to_float(ast) do
    traverse_expression(ast, fn
      {:integer, value} ->
        {:ok, {:float, value <> ".0"}}

      _other ->
        :error
    end)
  end

  def coerce_to_int(ast) do
    traverse_expression(ast, fn
      {:float, value} ->
        rounded =
          value
          |> String.to_float()
          |> round()
          |> to_string()

        {:ok, {:integer, rounded}}

      _other ->
        :error
    end)
  end

  def maybe_coerce_type(func_arg, given_arg) do
    cond do
      func_arg.type in ["f32", "f64"] ->
        coerce_to_float(given_arg)


      func_arg.type in ["i32", "i64", "u32", "u64"] ->
        coerce_to_int(given_arg)

      true ->
        given_arg
    end
  end

  def convert_sf_e_tests_to_rust(sf_tests, functions) do
    for {c_func, c_tests} <- sf_tests, into: %{} do
      function = Map.fetch!(functions, c_func)
      rust_name = function.rust_name

      valid_tests =
        c_tests
        |> Enum.reject(fn c_test -> c_test.expected == {:variable, "GSL_NAN"} end)
        |> Enum.reject(fn c_test -> c_test.tolerance == {:variable, "GSL_NAN"} end)
        |> Enum.reject(fn c_test ->
            Enum.any?(c_test.args, fn arg -> arg == {:variable, "x"} or arg == {:variable, "n"} end)
          end)

      rust_tests =
        for c_test <- valid_tests do
          rust_args_without_refs = Enum.reject(c_test.args, fn arg -> match?({:ref, _}, arg) end)

          rust_args =
            for {func_rust_arg, given_rust_arg} <- Enum.zip(function.rust_arguments, rust_args_without_refs) do
              traverse_expression(
                maybe_coerce_type(func_rust_arg, given_rust_arg),
                fn ast ->
                  case ast do
                    {:variable, "GSL_DBL_EPSILON"} ->
                      {:ok, {:variable, "DBL_EPSILON"}}

                    _other ->
                      :error
                  end
                end
              )
            end

          dummy_rust_arg = %RustFunctionArg{type: "f64", name: "dummy"}

          rust_expected = maybe_coerce_type(dummy_rust_arg, c_test.expected)

          rust_tolerance = maybe_coerce_type(dummy_rust_arg, c_test.tolerance)

          %{
              c_test |
              function_name: rust_name,
              args: rust_args,
              expected: rust_expected,
              tolerance: rust_tolerance
          }
        end

      {rust_name, rust_tests}
    end
  end

  def build_gsl_sf_module(functions, sf_tests, gsl_submodule) do
    module_functions = filter_functions(functions, fn f ->
      f.gsl_submodule == gsl_submodule
    end)

    sf_e_tests_for_module =
      sf_tests
      |> Map.take(Enum.map(module_functions, fn f -> f.c_name end))
      |> convert_sf_e_tests_to_rust(functions)

    assigns =
      module_functions
      |> group_sf_functions()
      |> Map.put_new(:sf_e_functions, [])
      |> Map.put_new(:sf_normal_functions, [])
      |> Map.put(:sf_e_tests, sf_e_tests_for_module)

    transformed_sf_e_functions =
      for sf <- assigns[:sf_e_functions] do
        transform_sf_e_function(sf)
      end

    assigns = %{assigns | sf_e_functions: transformed_sf_e_functions}

      render_template_to_file(
        "templates/rs/special/#{gsl_submodule}.rs",
        "../src/special/#{gsl_submodule}.rs",
        assigns
      )
  end

  def clone_sf_master_template(submodules) do
    template_content = File.read!("templates/rs/special/MASTER.rs")

    for submodule <- submodules do
      new_template_content = String.replace(template_content, "$MASTER", submodule)
      File.write!("templates/rs/special/#{submodule}.rs", new_template_content)
    end
  end

  def build_randist(functions) do
    ran_functions = filter_functions(functions, fn f ->
      case f.rust_arguments do
        [arg | _others] ->
          f.c_module == "randist" and arg.type == "Rng"

        _other ->
          false
      end
    end)

    unordered_normal_functions = filter_functions(functions, fn f ->
      case f.rust_arguments do
        [arg | _others] ->
          f.c_module == "randist" and arg.type != "Rng"

        _other ->
          false
      end
    end)

    ordered_normal_functions = Enum.sort_by(unordered_normal_functions, fn f -> f.rust_name end)

    normal_functions =
      for func <- ordered_normal_functions do
        %{func | doc: String.replace(func.doc, ", using the formula given above.", ".")}
      end

    ran_functions =
      for ran_fun <- ran_functions do
        [_ | new_rust_args] = ran_fun.rust_arguments
        [_ | _new_c_args] = ran_fun.rust_arguments

        %{ran_fun | rust_arguments: new_rust_args}
      end

    assigns = %{
      ran_functions: ran_functions,
      normal_functions: normal_functions
    }

    render_template_to_file(
      "templates/rs/distribution.rs",
      "../src/distribution.rs",
      assigns
    )
  end

  def setup() do
    sf_modules =
      for path <- Path.wildcard("../gsl/specfunc/*.h"),
          String.starts_with?(Path.basename(path), "gsl_sf_"),
          do: path_to_submodule(path)

    clone_sf_master_template(sf_modules)

    :ok
  end

  def extract_sf_tests() do
    list =
      Enum.flat_map(Path.wildcard("../gsl/specfunc/test_*.c"), fn path ->
        path
        |> File.read!()
        |> RSci.SfTestParser.parse()
      end)

    Enum.group_by(list, fn sf_test -> sf_test.function_name end)
  end

  def render_template_to_file(template_path, destination_path, assigns) do
    template_content = File.read!(template_path)

    file_contents =
      EEx.eval_string(
        template_content,
        [assigns: assigns],
        engine: EEx.SmartEngine
      )

    File.write!(destination_path, file_contents)
  end

  def run() do
    functions = extract_functions_for_all_gsl_modules()

    sf_tests = extract_sf_tests()

    submodules =
      for path <- Path.wildcard("../gsl/specfunc/*.h"),
          String.starts_with?(Path.basename(path), "gsl_sf_"),
          Path.basename(path, ".h") not in ["result", "airy"],
          do: path_to_submodule(path)

    build_randist(functions)

    for submodule <- submodules do
      build_gsl_sf_module(functions, sf_tests, submodule)
    end

    :ok
  end
end
