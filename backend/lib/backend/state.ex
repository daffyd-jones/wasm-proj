defmodule Backend.State do
  use GenServer
  alias Backend.{
    PlayerSequence,
    PlayerSupervisor,
    Player,
    State,
    Grid,
    Bomb,
    Wall
  }

  defstruct bombs: [],
            walls: []

  ### Public API ###

  def start_link(nil) do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def inspect() do
    GenServer.call(__MODULE__, :inspect)
  end

  def finish_turn(uuid, new_state) do
    GenServer.call(__MODULE__, {:finish_turn, uuid, new_state})
  end
 
  ### GenServer ###

  @impl true
  def init(_) do
    {:ok, %State{bombs: [], walls: []}}
  end

  @impl true
  def handle_call(:inspect, _from, state) do
    reply = Map.merge(
      %{players: PlayerSupervisor.inspect_all()},
      state |> Map.from_struct()
    )
    {:reply, reply, state}
  end

  defp state_from_map(%{"bombs" => bombs, "walls" => walls}) do
    %State{
      bombs: bombs |> Enum.map(fn x -> Bomb.from_map(x) end),
      walls: walls |> Enum.map(fn x -> Wall.from_map(x) end)
    } 
  end
  
  @impl true
  def handle_call({:finish_turn, uuid, new_state}, _from, state) do
    case PlayerSequence.current_plr() do
      ^uuid ->
        IO.inspect("It is #{uuid}'s turn")

        PlayerSequence.save_next_plr()
        PlayerSupervisor.update_plrs(new_state["players"])
        
        #%{"bombs" => bombs, "walls" => walls} = new_state
        #new_server_state = %State{
        #  bombs: bombs |> Enum.map(fn x -> Bomb.from_map(x) end),
        #  walls: walls |> Enum.map(fn x -> Wall.from_map(x) end)
        #}

        # Sending the next player ID
        {:reply, PlayerSequence.current_plr(), state_from_map(new_state)}
      _ ->
        IO.inspect("It is not #{uuid}'s turn")
        
        {:reply, :not_your_turn, state}
    end
  end
end
