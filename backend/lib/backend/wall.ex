defmodule Wall do
  @derive {Jason.Encoder, exclude: []}
  defstruct x: 0,
            y: 0,
            destructible: true,
            alive: true
end
