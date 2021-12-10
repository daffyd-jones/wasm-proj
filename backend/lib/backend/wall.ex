defmodule Backend.Wall do
  @derive {Jason.Encoder, exclude: []}
  defstruct x: 0,
            y: 0,
            destructible: true,
            alive: true

  def from_map(%{
    "x" => x,
    "y" => y,
    "destructible" => destructible,
    "alive" => alive}) 
  do
    %Backend.Wall{x: x, y: y, destructible: destructible, alive: alive}
  end
end
