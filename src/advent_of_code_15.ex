defmodule Disc do
  @enforce_keys [:positions, :start]
  defstruct [:positions , :start]
end

defmodule Adventofcode15 do


  @moduledoc """
  Documentation for Adventofcode15.
  """
  def start() do
      case File.read("puzzleinput.txt") do
      {:ok, body} -> getTiming(parse(body), 0)
      {:error, reason} -> IO.puts("Problem reading Puzzle input. " + reason)
      end
  end

  def start2() do
      case File.read("puzzleinput.txt") do
      {:ok, body} -> getTiming(parse(body) ++ [%Disc{positions: 11, start: 0}], 0)
      {:error, reason} -> IO.puts("Problem reading Puzzle input. " + reason)
      end
  end

  def parse(body) do
    String.split(body, "\n")
    |> Enum.map(fn line -> 
      [_discNo, pos, _time, startVal] = List.flatten(Regex.scan(~r{\d+}, line))
      %Disc{positions: String.to_integer(pos), start: String.to_integer(startVal)}
    end)
  end


  def getTiming(discs, time) do
    if(rem(time, 1000) == 0) do
      IO.inspect(time)
    end
    if passesThrough?(discs, time) do
      time
    else
      getTiming(discs, time + 1)
    end
  end
  
  def passesThrough?([], time), do: true
  def passesThrough?([head | tail], time) do
    if(blocks?(head, time+1)) do
      false
    else
      passesThrough?(tail, time + 1)
    end
  end

  defp blocks?(%Disc{positions: positions, start: start}, time) do
    rem(start + time, positions) != 0
    end

end
