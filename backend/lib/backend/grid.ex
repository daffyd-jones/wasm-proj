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
  
  def to_list_remove_keys(map) do
    map
    |> Enum.to_list()
    |> Enum.map(fn {_, x} -> x end)
  end
  
  # Warning: After a certain size, maps become unordered
  def to_list(grid) do
    grid
    |> to_list_remove_keys()
    |> Enum.map(&to_list_remove_keys/1)
  end
end
