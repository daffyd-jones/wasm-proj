defmodule Backend.Grid do
  def new(width, height) do
    0..(height - 1)
    |> Enum.map(fn y ->
      {y, 0..(width - 1)|> Enum.map(fn x -> {x, nil} end) |> Map.new()}
    end)
    |> Map.new()
  end

  def set(grid, x, y, val) do
    put_in(grid[x][y], val)
  end
end
