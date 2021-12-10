defmodule Backend.Player do
  use GenServer
  alias BackendWeb.Endpoint

  # Structs are named after the module they are in
  # so we have to create another module
  defmodule State do
    @derive {Jason.Encoder, except: []}
    defstruct id: nil,
              x: 1,
              y: 1,
              bombs: 10,
              hp: 0,
              alive: true
    
    def from_map(%{
      "id" => id,
      "x" => x,
      "y" => y,
      "bombs" => bombs,
      "hp" => hp,
      "alive" => alive
    }) do
      %State{
        id: id,
        x: x,
        y: y,
        bombs: bombs,
        hp: hp,
        alive: alive
      }
    end
  end

  ### Public API ###
  
  def start_link(uuid) do
    GenServer.start_link(__MODULE__, uuid, name: name(uuid))
  end
  
  defp name(uuid) do
    {:global, uuid}
  end

  def inspect(uuid) when is_integer(uuid) do
    GenServer.call(name(uuid), :inspect)
  end

  def inspect(pid), do: GenServer.call(pid, :inspect)

  def update_state(uuid, new_state) do
    GenServer.cast(name(uuid), {:update_state, new_state})
  end

  ### GenServer ###
  
  @impl true
  def init(uuid) do
    state = %State{id: uuid}
    IO.puts("New player process created with UUID #{uuid}.")
    
    {:ok, state}
  end
  
  @impl true
  def handle_call(:inspect, _from, state) do
    {:reply, state, state}
  end

  @impl true
  def handle_cast({:update_state, new_state}, _state) do
    {:noreply, State.from_map(new_state)}
  end
end
