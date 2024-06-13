defmodule RSci.MixProject do
  use Mix.Project

  def project do
    [
      app: :rsci,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger, :inets, :ssl, :eex]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:nimble_parsec, "~> 1.4"},
      {:floki, "~> 0.36"},
      {:excribe, "~> 0.1"}
    ]
  end
end
