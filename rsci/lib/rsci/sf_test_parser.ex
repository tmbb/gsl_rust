
defmodule RSci.SfTest do
  defstruct tag: nil,
            function_name: nil,
            args: nil,
            expected: nil,
            tolerance: nil

  def build([{tag, {:tuple, macro_args}}]) do
    [
      _status,
      {:variable, function_name},
      {:tuple, fun_args},
      expected,
      tolerance,
      _gsl_status
    ] = macro_args

    %__MODULE__{
      tag: tag,
      function_name: function_name,
      args: fun_args,
      expected: expected,
      tolerance: tolerance
    }
  end

  def commas(elements), do: elements |> Enum.map(&serialize/1) |> Enum.intersperse(", ")

  def serialize({:tuple, args}), do: ["(", commas(args), ")"]
  def serialize({basic_tag, var}) when basic_tag in [:integer, :float, :variable], do: var
  def serialize({:ref, expr}), do: ["&", serialize(expr)]
  def serialize({:parenthesis, expr}), do: ["(", serialize(expr), ")"]

  def serialize({:function_call, f, args}), do: [f, "(", commas(args), ")"]

  def serialize({:+, [a, b]}), do: [serialize(a), " + ", serialize(b)]
  def serialize({:-, [a, b]}), do: [serialize(a), " + ", serialize(b)]
  def serialize({:*, [a, b]}), do: [serialize(a), "*", serialize(b)]
  def serialize({:/, [a, b]}), do: [serialize(a), "/", serialize(b)]
end

defmodule RSci.SfTestParser do
  import NimbleParsec

  def build_function_call([f | args]) do
    {:function_call, f, args}
  end

  whitespace = ascii_string([?\s, ?\t, ?\n], min: 1)

  seq = fn [combinator | combinators] ->
    Enum.reduce(combinators, ignore(optional(whitespace)) |> concat(combinator), fn comb, acc ->
      acc
      |> ignore(optional(whitespace))
      |> concat(comb)
    end)
    |> ignore(optional(whitespace))
  end

  identifier =
    ascii_char([?a..?z, ?A..?Z, ?_])
    |> ascii_string([?a..?z, ?A..?Z, ?0..?9, ?_], min: 0)
    |> reduce({IO, :iodata_to_binary, []})

  integer =
    optional(choice([ignore(string("+")), string("-")]))
    |> ascii_string([?0..?9], min: 1)
    |> reduce({IO, :iodata_to_binary, []})
    |> unwrap_and_tag(:integer)

  float =
    optional(choice([ignore(string("+")), string("-")]))
    |> ascii_string([?0..?9], min: 1)
    |> string(".")
    |> ascii_string([?0..?9], min: 1)
    |> optional(
      string("e")
      |> optional(ascii_char([?+, ?-]))
      |> ascii_string([?0..?9], min: 1)
    )
    |> reduce({IO, :iodata_to_binary, []})
    |> unwrap_and_tag(:float)

  number = choice([float, integer])

  function_call =
    seq.([
      identifier,
      ignore(string("(")),
      optional(parsec(:expr)),
      repeat(seq.([ignore(string(",")), parsec(:expr)])),
      ignore(string(")"))
    ])
    |> reduce(:build_function_call)

  variable = identifier |> unwrap_and_tag(:variable)

  ref = ignore(string("&")) |> concat(parsec(:expr)) |> unwrap_and_tag(:ref)

  parenthesis =
    seq.([
      ignore(string("(")),
      parsec(:expr),
      ignore(string(")")),
    ])
    |> unwrap_and_tag(:parenthesis)

  tuple =
    seq.([
      ignore(string("(")),
      optional(parsec(:expr)),
      repeat(seq.([ignore(string(",")), parsec(:expr)])),
      ignore(string(")"))
    ])
    |> tag(:tuple)

  arith_monomial = choice([
    parsec(:arith_mul),
    parsec(:arith_div)
  ])

  defcombinatorp :arith_add, seq.([
    choice([
      arith_monomial,
      parsec(:non_arith_expr)
    ]),
    ignore(string("+")),
    choice([
      arith_monomial,
      parsec(:non_arith_expr)
    ]),
  ])
  |> tag(:+)

  defcombinatorp :arith_sub, seq.([
    choice([
      arith_monomial,
      parsec(:non_arith_expr)
    ]),
    ignore(string("-")),
    choice([
      arith_monomial,
      parsec(:non_arith_expr)
    ]),
  ])
  |> tag(:-)

  defcombinatorp :arith_mul, seq.([
    parsec(:non_arith_expr),
    ignore(string("*")),
    choice([
      arith_monomial,
      parsec(:non_arith_expr)
    ])
  ])
  |> tag(:*)

  defcombinatorp :arith_div, seq.([
    parsec(:non_arith_expr),
    ignore(string("/")),
    choice([
      arith_monomial,
      parsec(:non_arith_expr)
    ])
  ])
  |> tag(:/)

  defcombinatorp :arith_expr, choice([
    parsec(:arith_add),
    parsec(:arith_sub),
    parsec(:arith_mul),
    parsec(:arith_div)
  ])

  defcombinatorp :non_arith_expr, choice([
    function_call,
    ref,
    parenthesis,
    tuple,
    variable,
    number
  ])

  defcombinatorp :expr, choice([
    parsec(:arith_expr),
    parsec(:non_arith_expr)
  ])

  test_sf_call =
    seq.([
      string("TEST_SF"),
      tuple,
      ignore(string(";"))
    ])
    |> reduce({List, :to_tuple, []})
    |> reduce({RSci.SfTest, :build, []})

  test_sf_calls =
    repeat(
      choice([
        test_sf_call,
        ignore(utf8_char([]))
      ])
    )

  defparsec :root, test_sf_calls

  def normalize_to_unix(text) do
    String.replace(text, "\r\n", "\n")
  end

  def parse(text) do
    case root(text) do
      {:ok, results, _, _, _, _} ->
        results

      _error ->
        []
    end
  end

  def run() do
    path = "../gsl/specfunc/test_mathieu.c"
    text = File.read!(path) |> normalize_to_unix()

    # text = """
    # TEST_SF(s, gsl_sf_mathieu_ce_e, (0, 0.0, 0.0, &r),
    #       0.7071067811865475, TEST_SNGL, GSL_SUCCESS);
    # """

    parse(text)
  end
end
