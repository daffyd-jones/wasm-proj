defmodule Backend.Bomb do
  @derive {Jason.Encoder, exclude: []}
  defstruct x: 0,
            y: 0,
            power: 0,
            timer: 0
end
