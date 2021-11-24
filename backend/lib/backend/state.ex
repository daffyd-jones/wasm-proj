defmodule Backend.State do
  use GenServer
  alias Backend.{PlayerSupervisor, Player, Grid}
  
  defmodule Grids do
    @width 3
    @height 3

    defstruct blocks: %{},
              players: %{},
              bombs: %{}

    def new() do
      %Grids{
        blocks: Grid.new(@width, @height),
        players: Grid.new(@width, @height),
        bombs: Grid.new(@width, @height),
      }
    end
  end
  
  defimpl Jason.Encoder, for: Grids do
    def encode(map, opts) do
      # In this case, our grids are nested maps.
      # We want to convert them so the parser
      # can return a 2D array in JSON
      converted_map = map
                      |> Map.from_struct()
                      |> Map.to_list()
                      |> Enum.map(fn {k, v} -> {k, Grid.to_list(v)} end)
                      |> Map.new()
      Jason.Encode.map(converted_map, opts)
    end
  end
  
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
    # state = %{
    #   blocks: Grid.new(@width, @height),
    #   players: Grid.new(@width, @height),
    #   bombs: Grid.new(@width, @height)
    # }

    {:ok, Grids.new()}
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
