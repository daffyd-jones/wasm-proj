defmodule Backend.State do
  use GenServer
  alias Backend.{PlayerSupervisor, Player}

  # this will probably store stuff like bombs

  def start_link(nil) do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
  end

  @impl true
  def init(_) do
    {:ok, nil}
  end
end
