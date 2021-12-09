defmodule Backend.Bomb do
  @derive {Jason.Encoder, exclude: []}
  defstruct x: 0,
            y: 0,
            power: 0,
            timer: 0

  def from_map(
    %{"x" => x, "y" => y, "power" => power, "timer" => timer}
  ) do
    %Backend.Bomb{x: x, y: y, power: power, timer: timer}
  end
end
