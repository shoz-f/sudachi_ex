defmodule Sudachi.MixProject do
  use Mix.Project

  def project do
    [
      app: :sudachi,
      version: "0.1.0",
      elixir: "~> 1.13-rc",
      start_permanent: Mix.env() == :prod,
      deps: deps(),

      description: description(),
      package: package(),
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.22.2"},
      {:ex_doc, "~> 0.24", only: :dev, runtime: false},
    ]
  end

  defp description() do
    "Japanese morphological analyzer Sudachi."
  end

  defp package() do
    [
      name: "sudachi",
      licenses: ["Apache-2.0"],
      links: %{"GitHub" => "https://github.com/shoz-f/sudachi_ex.git"},
      files: ["lib", "priv", "mix.exs", "README*", "LICENSE*", "native"]
    ]
  end
end
