defmodule Backend.PlayerSupervisor do
  use DynamicSupervisor
  alias Backend.{Player, PlayerSequence}

  def start_link(_) do
    DynamicSupervisor.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def start_player(uuid) do
    case DynamicSupervisor.start_child(__MODULE__, {Player, uuid}) do
      {:ok, _} -> PlayerSequence.add_plr(uuid)
      err -> err
    end
  end

  def init(_) do
    DynamicSupervisor.init(strategy: :one_for_one)
  end

  def update_plrs(new_states) do
    new_states
    |> Enum.each(fn x -> Player.update_state(x["id"], x) end)
  end

  def inspect_all() do
    DynamicSupervisor.which_children(__MODULE__)
    |> Enum.map(fn {_, pid, _, _} -> Player.inspect(pid) end)
  end
end
