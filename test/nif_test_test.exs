defmodule NifTestTest do
  use ExUnit.Case
  doctest NifTest

  test "NifTest.add/2 NIF" do
    assert NifTest.add(2, 9) == {:ok, 11}
  end

  test "NifTest.message/1 NIF" do
    assert NifTest.println("hello world") == :ok
  end
end
