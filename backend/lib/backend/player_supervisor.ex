defmodule Backend.PlayerSupervisor do
  use DynamicSupervisor
  alias Backend.Player

  def start_link(_) do
    DynamicSupervisor.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def start_player(uuid) do
    DynamicSupervisor.start_child(__MODULE__, {Player, uuid})
  end

  def init(_) do
    DynamicSupervisor.init(strategy: :one_for_one)
  end

  def inspect_all() do
    DynamicSupervisor.which_children(__MODULE__)
    |> Enum.map(fn {_, pid, _, _} -> Player.inspect(pid) end)
  end
end
