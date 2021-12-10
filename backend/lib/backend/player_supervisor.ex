defmodule Backend.PlayerSupervisor do
  use DynamicSupervisor
  alias Backend.{Player, PlayerSequence}

  def start_link(_) do
    DynamicSupervisor.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def start_player(uuid) do
    case DynamicSupervisor.start_child(__MODULE__, {Player, uuid}) do
      {:ok, _} -> 
        PlayerSequence.add_plr(uuid)
        # Changing the player's position depending on their
        # turn order. Poor planning on my part makes everything
        # scuffed hehe
        pos = case PlayerSequence.inspect().players |> length() do
          1 -> %{x: 1, y: 1}
          2 -> %{x: 31, y: 31}
        end

        new_state = Map.merge(Player.inspect(uuid), pos)
        
        Player.set_pos(uuid, new_state)

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
