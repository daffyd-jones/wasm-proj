defmodule BackendWeb.RoomChannel do
  use BackendWeb, :channel
  alias Backend.{
    State,
    PlayerSupervisor,
    PlayerSequence,
    Player
  }
  
  @impl true
  def join("room:lobby", payload, socket) do
    %{"uuid" => uuid} = payload
    
    case PlayerSupervisor.start_player(uuid) do
      :ok ->
        # Send self a message because we can't broadcast
        # before the socket fully joins
        send(self(), :after_join)
        # Attaches the UUID to the socket to associate the socket
        # with the newly created Player process
        {:ok, assign(socket, :uuid, uuid)}
      err ->
        IO.inspect(err)
        {:error, err}
    end
  end
  
  @impl true
  def handle_info(:after_join, socket) do
    state = Player.inspect(socket.assigns.uuid)

    # Let each player that's already connected know
    # that a new player just connected
    broadcast socket, "new_plr", state

    # TODO: Send client the state of all players + game
    {:noreply, socket}
  end

  @impl true
  def handle_in("inspect", _payload, socket) do
    state = Player.inspect(socket.assigns.uuid)
    {:reply, {:ok, state}, socket}
  end

  @impl true
  def handle_in("inspect_all", _payload, socket) do
    plr_states = PlayerSupervisor.inspect_all()
    {:reply, {:ok, plr_states}, socket}
  end

  # Still unclear on how game stuff works still
  # I presume that the player will send its 
  # entire internal state to the server
  @impl true
  def handle_in("next_turn", payload, socket) do
    # Payload is just %{"uuid" => whatever} for now
    case State.next_turn(socket.assigns.uuid, payload) do
      :not_your_turn -> nil
      uuid -> 
        broadcast_from socket, "new_turn",
          # sending status for testing purposes
          %{status: :new_turn, uuid: uuid}
    end
    
    {:reply, :ok, socket}
  end

  @impl true
  def handle_in("update_pos", payload, socket) do
    # Ignore the logging for update_pos since it
    # spams IEX
    Logger.disable(self())
    
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
