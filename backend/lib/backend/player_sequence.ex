defmodule Backend.PlayerSequence do
  use GenServer
  alias Backend.{PlayerSequence, PlayerSupervisor}

  # This keeps track of the sequence of players for turns
  # and stuff like that idk
  # Only stores UUIDs
  defstruct players: [],
            current: nil
  
  ### Public API ###

  def start_link(_) do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def inspect(), do: GenServer.call(__MODULE__, :inspect)

  def current_plr(), do: GenServer.call(__MODULE__, :current_plr)

  def get_next_plr(), do: GenServer.call(__MODULE__, :get_next_plr)

  def save_next_plr(), do: GenServer.call(__MODULE__, :save_next_plr)

  def add_plr(uuid), do: GenServer.cast(__MODULE__, {:add_plr, uuid})
  
  # for testing purposes 
  def divide_by_zero, do: GenServer.cast(__MODULE__, :divide_by_zero)

  ### GenServer ###

  @impl true
  def init(nil) do
    uuids = PlayerSupervisor.inspect_all()
            |> Enum.map(fn x -> x.uuid end)
    state = %PlayerSequence{
      players: uuids,
      current: nil
    }
    {:ok, state}
  end

  def handle_call(:inspect, _from, state), do: {:reply, state, state}

  @impl true
  def handle_call(:current_plr, _from, state) do
    {:reply, state.current, state}
  end
  
  defp get_new_current(state) do
    case state.current do
      nil ->
        state.players |> Enum.at(0)
      _ ->
        cur_i = state.players
                |> Enum.find_index(fn x -> x == state.current end)
        next_i = rem(cur_i + 1, state.players |> Enum.count())
        state.players |> Enum.at(next_i)
    end
  end

  @impl true
  def handle_call(:get_next_plr, _from, state) do
    new_current = get_new_current(state)
    {:reply, new_current, state}
  end

  @impl true
  def handle_call(:save_next_plr, _from, state) do
    new_current = get_new_current(state)

    new_state = %PlayerSequence{
      state | 
      current: new_current
    }

    {:reply, new_state.current, new_state}
  end

  @impl true
  def handle_cast({:add_plr, uuid}, state) do
    new_state = %PlayerSequence{
      state |
      players: state.players ++ [uuid]
    }
    {:noreply, new_state}
  end

  @impl true
  def handle_cast(:divide_by_zero, _state) do
    1/0
  end
end
