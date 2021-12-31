# sudachi_ex

sudachi_ex is the Elixir binding of Sudachi, the Japanese morphological analyzer.<br>

** *EXPERIMENTAL* **

## Installation

the sudachi_ex is installed by adding `sudachi` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:sudachi, "~> 0.1.0"}
  ]
end
```

You need to featch a sudachi dictionary:

```shell
mix sudachi.fetch_dic
```

## Try it

Now you can try Sudachi -the Japanese morphological analyzer- in Elixir!<br>
Enjoy ;-)

```elixir
$ iex -S mix
Erlang/OTP 24 [erts-12.1.4] [source] [64-bit] [smp:8:8] [ds:8:8:10] [async-threads:1] [jit]

Interactive Elixir (1.13.0-rc.0) - press Ctrl+C to exit (type h() ENTER for help)
iex(1)> Sudachi.analize("打込む かつ丼 附属 vintage")
[
  ["打込む", "動詞", "一般", "*", "*", "五段-マ行",
   "終止形-一般", "打ち込む"],
  [" ", "空白", "*", "*", "*", "*", "*", " "],
  ["かつ丼", "名詞", "普通名詞", "一般", "*", "*", "*", "カツ丼"],
  [" ", "空白", "*", "*", "*", "*", "*", " "],
  ["附属", "名詞", "普通名詞", "サ変可能", "*", "*", "*", "付属"],
  [" ", "空白", "*", "*", "*", "*", "*", " "],
  ["vintage", "名詞", "普通名詞", "一般", "*", "*", "*",
   "ビンテージ"]
]
```

## License
sudachi_ex is licensed under the Apache License Version 2.0.

#### -- license overview of included 3rd party libraries --
- The "sudachi.rs" library is licensed under the Apache License Version 2.0.
- The "sudachi-dictionary-20210802-core" is licensed under the Apache License Version 2.0.
