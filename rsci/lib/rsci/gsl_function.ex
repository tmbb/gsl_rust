defmodule RSci.GslFunction do
  defstruct c_name: nil,
            rust_name: nil,
            c_module: nil,
            rust_module: nil,
            c_arguments: nil,
            rust_arguments: nil,
            c_return_type: nil,
            rust_return_type: nil,
            gsl_path: nil,
            gsl_submodule: nil,
            doc: nil
end

defmodule RSci.GslFunctionArg do
  defstruct type: nil,
            name: nil
end

defmodule RSci.RustFunctionArg do
  defstruct type: nil,
            name: nil
end

defmodule RSci.GslFunctionDefParser do
  import NimbleParsec

  whitespace = ascii_string([?\s, ?\t, ?\n], min: 1)

  space_separated_type_identifiers = fn line ->
    [ident | idents] = String.split(line)

    parser =
      Enum.reduce(idents, string(ident), fn next_ident, acc ->
        acc
        |> ignore(whitespace)
        |> string(next_ident)
      end)

    parser
  end

  seq = fn [combinator | combinators] ->
    Enum.reduce(combinators, ignore(optional(whitespace)) |> concat(combinator), fn comb, acc ->
      acc
      |> ignore(optional(whitespace))
      |> concat(comb)
    end)
    |> ignore(optional(whitespace))
  end

  valid_basic_types = """
  char
  signed char
  unsigned char
  short
  short int
  signed short
  signed short int
  unsigned short
  unsigned short int
  int
  signed
  signed int
  unsigned
  unsigned int
  long
  long int
  signed long
  signed long int
  unsigned long
  unsigned long int
  long long
  long long int
  signed long long
  signed long long int
  unsigned long long
  unsigned long long int
  float
  double
  long double
  """
  |> String.split("\n")
  # Sort and reverse teh items so that longer strings
  # will match before their prefixes
  |> Enum.sort()
  |> Enum.reverse()
  |> Enum.filter(fn line -> line != "" end)
  |> Enum.map(fn line -> space_separated_type_identifiers.(line) end)

  array_brackets = seq.([string("["), string("]")]) |> reduce({Enum, :join, []})

  generic_identifier = ascii_string([?a..?z, ?A..?Z, ?0..?9, ?_], min: 1)

  function_identifier = generic_identifier
  type_identifier = generic_identifier
  arg_identifier = generic_identifier

  type =
    seq.([
      optional(string("const")),
      choice(valid_basic_types ++ [type_identifier]),
      repeat(string("*") |> ignore(optional(whitespace)))
    ])
    |> reduce({Enum, :join, [" "]})

  def handle_optional_brackets(items) do
    case items do
      [type, arg] -> {type, arg}
      [type, arg, brackets] -> {type <> " " <> brackets, arg}
    end
  end

  arg =
    seq.([
      type,
      arg_identifier,
      optional(array_brackets)
    ])
    |> reduce(:handle_optional_brackets)

  argument_list =
    seq.([
      optional(arg),
      repeat(
        seq.([
          ignore(string(",")),
          arg
        ])
      )
    ])
    |> wrap()

  function_definition =
    seq.([
      type,
      function_identifier,
      ignore(string("(")),
      argument_list,
      ignore(string(")"))
    ])
    |> reduce({List, :to_tuple, []})


  defparsec :root, function_definition

  def parse(text) do
    case root(text) do
      {:ok, [result], _remainder, _, _, _} ->
        {:ok, result}

      other ->
        other
    end
  end
end
