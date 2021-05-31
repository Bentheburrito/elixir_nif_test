defmodule NifTestTest do
  use ExUnit.Case
  doctest NifTest

  test "greets the world" do
    assert NifTest.hello() == :world
  end
end
