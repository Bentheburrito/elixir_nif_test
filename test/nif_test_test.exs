defmodule NifTestTest do
  use ExUnit.Case
  doctest NifTest

  test "NifTest.add/2 NIF" do
    assert NifTest.add(2, 9) == {:ok, 11}
  end
end
