defmodule Backend.Player do
  use GenServer

  defp name(uuid) do
    {:global, uuid}
  end
  
  def do_something_funny(uuid) do
    GenServer.cast(name(uuid), :do_something_funny)
  end
  
  def start_link(uuid) do
    # TODO: wtf do i put for state??? lol
    GenServer.start_link(__MODULE__, uuid, name: name(uuid))
  end

  def init(uuid) do
    IO.puts("I AM ALIVE #{uuid}")
    {:ok, uuid}
  end
  
  @impl true
  def handle_cast(:do_something_funny, state) do
    IO.puts("Doing something funny!!! #{state}")
    {:noreply, state}
  end
end
