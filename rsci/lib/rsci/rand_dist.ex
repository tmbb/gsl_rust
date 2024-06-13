defmodule RSci.RandDist do
  alias RSci.GslFunction

  def nr_of_samples_var_name() do
    "nr_of_samples"
  end

  def create_draw_samples_functions(functions) do
    functions
    |> Enum.filter(fn f -> String.starts_with?(f.rust_name, "draw_sample_from_") end)
    |> Enum.map(&convert_draw_sample_function_to_plural/1)
  end

  defp convert_draw_sample_function_to_plural(%GslFunction{} = function) do
    "draw_sample_from_" <> base_name = function.rust_name
    new_rust_name = base_name

    new_doc =
      function.doc
      |> String.replace(
          "returns a random variate",
          "returns `#{nr_of_samples_var_name()}` random variates"
        )
      |> String.replace(
          "a Gaussian random variate",
          "returns `#{nr_of_samples_var_name()}` Gaussian random variates"
        )
      |> String.replace(
          "returns a random integer",
          "returns `#{nr_of_samples_var_name()}` random integers"
        )
      |> String.replace(
          "the result of a Bernoulli trial",
          "the result of `#{nr_of_samples_var_name()}` Bernoulli trials"
        )

    %{function | rust_name: new_rust_name, doc: new_doc}
  end
end
