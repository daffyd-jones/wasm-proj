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
    
    # Max 4 players
    if PlayerSequence.inspect().players |> length() > 1 do
      {:error, "Only two players allowed"}
    else
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
  end

  @impl true
  def handle_info(:after_join, socket) do
    state = Player.inspect(socket.assigns.uuid)

    # Let each player that's already connected know
    # that a new player just connected
    broadcast socket, "new_plr", state

    {:noreply, socket}
  end

  @impl true
  def handle_in("inspect_state", _payload, socket) do
    {:reply, {:ok, State.inspect()}, socket}
  end

  @impl true
  def handle_in("win", payload, socket) do
    %{"winner_id" => winner_id} = payload
    broadcast socket, "plr_wins", winner_id
    {:reply, :ok, socket}
  end

  @impl true
  def handle_in("finish_turn", payload, socket) do
    case State.finish_turn(socket.assigns.uuid, payload) do
      :not_your_turn -> nil
      next_player -> 
        broadcast socket, "new_turn",
          %{next_player: next_player, new_state: State.inspect()}
    end

    {:reply, :ok, socket}
  end
end
