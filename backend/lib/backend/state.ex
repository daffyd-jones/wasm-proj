defmodule Backend.State do
  use GenServer
  alias Backend.{PlayerSupervisor, Player, Grid}
  
  @width 3
  @height 3
  
  ### Public API ###

  def start_link(nil) do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
  end

  def get() do
    GenServer.call(__MODULE__, :get)
  end

  def set(grid, x, y, val) do
    GenServer.call(__MODULE__, {:set, grid, x, y, val})
  end

  ### GenServer ###

  @impl true
  def init(_) do
    state = %{
      blocks: Grid.new(@width, @height),
      players: Grid.new(@width, @height),
      bombs: Grid.new(@width, @height)
    }

    {:ok, state}
  end

  def handle_call({:set, grid, x, y, val}, _from, state) do
    new_state = state[grid][x][y] |> put_in(val)
    #new_state = state |> Map.put(grid, new_grid)
    {:reply, new_state, new_state}
  end
  
  @impl true
  def handle_call(:get, _from, state) do
    {:reply, state, state}
  end
end
