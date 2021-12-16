defmodule SudachiTest do
  use ExUnit.Case
  doctest Sudachi

  test "greets the world" do
    assert Sudachi.hello() == :world
  end
end
