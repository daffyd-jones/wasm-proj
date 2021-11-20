defmodule BackendWeb.RoomChannel do
  use BackendWeb, :channel
  alias Backend.{PlayerSupervisor, Player}

  @impl true
  def join("room:lobby", payload, socket) do
    %{"uuid" => uuid} = payload
    PlayerSupervisor.start_player(uuid)
    # Attaches the UUID to the socket to associate the socket
    # with the newly created Player process
    {:ok, assign(socket, :uuid, uuid)}
  end

  @impl true
  def handle_in("inspect", _payload, socket) do
    state = Player.inspect(socket.assigns.uuid)

    {:reply, {:ok, state}, socket}
  end
  
  @impl true
  def handle_in("update_pos", payload, socket) do
    %{"new_pos" => new_pos} = payload
    Player.update_pos(socket.assigns.uuid, new_pos)
   
    {:noreply, socket}
  end

  @impl true
  def handle_in("ping", payload, socket) do
    {:reply, {:ok, payload}, socket}
  end

  @impl true
  def handle_in("shout", payload, socket) do
    broadcast socket, "shout", payload
    {:noreply, socket}
  end
end
