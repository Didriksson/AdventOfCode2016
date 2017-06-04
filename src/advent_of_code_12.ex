defmodule Adventofcode12 do

  @moduledoc """
  Documentation for Adventofcode12.
  """

@doc """
Main entry to get the answer

You need to give the registers eg;
%{:a => 0, :b => 0, :c => 0, :d => 0}
"""

  def start(registers) do
      case File.read("puzzleinput.txt") do
      {:ok, body} -> handleInstructions(registers, String.split(body, "\n"), 0)
      {:error, reason} -> IO.puts("Problem reading Puzzle input. " + reason)
      end
  end

  @doc """
  Handles the instructions
  ## Examples

      iex> Adventofcode12.handleInstructions(%{:a => 0, :b => 0}, ["cpy 1 a", "cpy 2 b"], 0)
      %{a: 1, b: 2}

      iex> Adventofcode12.handleInstructions(%{:a => 0}, ["cpy 41 a", "inc a","inc a", "dec a", "jnz a 2", "dec a"], 0)
      %{a: 42}

  """
  def handleInstructions(registers, instructions, pointer) when pointer >= Kernel.length(instructions) do 
    registers
  end

  def handleInstructions(registers, instructions, pointer) do
    case String.split(Enum.at(instructions, pointer)) do
      ["cpy", val, reg] ->
        handleInstructions(cpy(registers, val, String.to_atom(reg)), instructions, pointer + 1)
      ["inc", reg] ->
        handleInstructions(inc(registers, String.to_atom(reg)), instructions, pointer + 1)
      ["dec", reg] ->
        handleInstructions(dec(registers, String.to_atom(reg)), instructions, pointer + 1)
      ["jnz", reg, val] ->
        handleInstructions(registers, instructions, pointer + jnz(registers, reg, val))
     end
  end

  @doc """
  Cpy for assembunny
  ## Examples

      iex> Adventofcode12.cpy(%{:a => 0}, 16, :a)
      %{a: 16}

      iex> Adventofcode12.cpy(%{:a => 0, :b => 6}, "b", :a)
      %{a: 6, b: 6}

  """
  def cpy(registers, value, into) do
    case Integer.parse(value) do
      {num, _} -> Map.put(registers, into, num)
      :error -> Map.put(registers, into, registers[String.to_atom(value)])
    end
  end

  @doc """
  inc for assembunny
  ## Examples

      iex> Adventofcode12.inc(%{:a => 0}, :a)
      %{a: 1}

      iex> Adventofcode12.inc(%{:a => 0, :b => 0}, :b)
      %{a: 0, b: 1}

  """
  def inc(registers, register) do
    Map.put(registers, register, registers[register] + 1)
  end

  @doc """
  dec for assembunny
  ## Examples

      iex> Adventofcode12.dec(%{:a => 1}, :a)
      %{a: 0}

  """
  def dec(registers, register) do
    Map.put(registers, register, registers[register] - 1)
  end 

  @doc """
  JNZ for assembunny
  ## Examples

      iex> Adventofcode12.jnz(%{:a => 1}, "a", "2")
      2
      iex> Adventofcode12.jnz(%{:a => 0}, "a", "2")
      0
      iex> Adventofcode12.jnz(%{:a => 1}, 2, "2")
      2
      iex> Adventofcode12.jnz(%{:a => 1}, 1, "5")
      5
  """
  def jnz(registers, num, val) when is_number(num) do
    if num == 0 do
      1
    else
      String.to_integer(val)
    end
  end

  def jnz(registers, register, val) do
    case Integer.parse(register) do
      {num, _} -> jnz(registers, num, val)
      :error -> jnz(registers, registers[String.to_atom(register)], val)
    end
  end
end
