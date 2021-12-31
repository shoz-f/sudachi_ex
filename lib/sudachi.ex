defmodule Sudachi do
  @moduledoc """
  sudachi_ex is the Elixir binding of Sudachi, the Japanese morphological analyzer.
  """

  alias Sudachi.NIF
  
  @doc """
  Analize the text and return the list of morphemes.

  ## Parameters

    * text - input text.
    * opts - analysis options
      - :all - Include extended analysis.
      - :debug - with debug outputs.

  ## Examples

    ```Elixir
    iex> res = Sudachi.analize("打込む かつ丼 附属 vintage")
    [
      ["打込む", "動詞", "一般", "*", "*", "五段-マ行", "終止形-一般", "打ち込む"],
      [" ", "空白", "*", "*", "*", "*", "*", " "],
      ["かつ丼", "名詞", "普通名詞", "一般", "*", "*", "*", "カツ丼"],
      [" ", "空白", "*", "*", "*", "*", "*", " "],
      ["附属", "名詞", "普通名詞", "サ変可能", "*", "*", "*", "付属"],
      [" ", "空白", "*", "*", "*", "*", "*", " "],
      ["vintage", "名詞", "普通名詞", "一般", "*", "*", "*", "ビンテージ"]
    ]
    ```
  """
  def analize(text, opts \\ []) do
    put_all      = (:all   in opts)
    enable_debug = (:debug in opts)
    NIF.analize(text, put_all, enable_debug)
  end
end
