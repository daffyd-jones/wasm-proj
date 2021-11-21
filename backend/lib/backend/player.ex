defmodule Backend.Player do
  use GenServer
  alias BackendWeb.Endpoint

  # Structs are named after the module they are in
  # so we have to create another module
  defmodule State do
    @derive {Jason.Encoder, except: []}
    defstruct name: "",
              uuid: nil,
              pos: %{x: 0, y: 0},
              bombs: 10,
              power: 10,
              lives: 3
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

  def update_pos(uuid, new_pos) do
    GenServer.cast(name(uuid), {:update_pos, new_pos})
  end

  ### GenServer ###
  
  @impl true
  def init(uuid) do
    state = %State{uuid: uuid}
    IO.puts("New player process created with UUID #{uuid}.")
    
    {:ok, state}
  end
  
  @impl true
  def handle_call(:inspect, _from, state) do
    {:reply, state, state}
  end

  @impl true
  def handle_cast({:update_pos, %{"x" => x, "y" => y}}, state) do
    new_state = %{state | pos: %{x: x, y: y}}
    Endpoint.broadcast("room:lobby", "new_pos", new_state)
    {:noreply, new_state}
  end
end
