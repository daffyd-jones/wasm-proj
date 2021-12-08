const thingy = document.getElementById("thingy");
const buttons = document.getElementById("buttons");
const clients = [];

const sendMsg = (socket, event, payload) => {
    socket.send(JSON.stringify({
        topic: "room:lobby",
        event: event,
        payload: payload,
        ref: 0
    }));
}

const createClient = (clientUUID) => {
    let socket = new WebSocket("ws://localhost:4000/socket/websocket");
    
    const client = {
        state: {
            players: [],
            bombs: [],
            walls: [],
        },
        socket: socket,
    };
    
    socket.onopen = (e) => {
        console.log(`${clientUUID} Joined :)`);
        sendMsg(socket, "phx_join", { uuid: clientUUID });
    }

    socket.onmessage = (e) => {
        const msg = JSON.parse(e.data);
        const payload = msg.payload;

        // Painful
        if (payload.status && payload.status === "new_turn") {
            thingy.innerText = `Player ${payload.uuid}'s turn`;
        } else if (msg.event === "new_plr") {
            
            if (payload.id === clientUUID) {
                sendMsg(socket, "inspect_all", {});
            } else {
                client.state.players.push(payload);
            }
        } else if (msg.event === "phx_reply") {
            
            // Matching the response thingy for "inspect_all"
            if (payload.response && Array.isArray(payload.response)) {
                client.state.players = payload.response;
            }
        }
    }

    return client;
}

const funnyJsonPayload = (state) => {
    const thingy = {
        "alive": true,
        ...state
    };

    return thingy;
}

for (i = 0; i < 4; i++) {
    const clientUUID = crypto.getRandomValues(new Uint32Array(1))[0];
    const b = document.createElement("button");
    b.innerText = `Player ${clientUUID}`;

    const client = createClient(clientUUID);
    clients.push(client);

    buttons.append(b);

    b.onclick = (e) => {
        console.log(b.innerText);
        sendMsg(client.socket, "next_turn", funnyJsonPayload(client.state));
    }
}

setTimeout(() => {
    for (i = 0; i < clients.length; i++) {
        console.log(clients[i].state);
    }
}, 1000);