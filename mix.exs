defmodule JemallocStats.MixProject do
  use Mix.Project

  def project do
    [
      app: :jemalloc_info,
      version: "0.4.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      docs: docs(),
      package: package()
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
      {:ex_doc, "~> 0.38.1", only: [:dev], runtime: false}
    ]
  end

  defp docs do
    [
      name: "JemallocInfo",
      extras: ["README.md"],
      main: "readme",
      source_url: "https://github.com/marinac-dev/jemalloc_info"
    ]
  end

  defp package do
    [
      name: :jemalloc_info,
      description: "A small library for exporting jemalloc allocation data in Elixir",
      maintainers: ["Marinac"],
      licenses: ["MIT"],
      links: %{
        "GitHub" => "https://github.com/marinac-dev/jemalloc_info"
      }
    ]
  end
end
