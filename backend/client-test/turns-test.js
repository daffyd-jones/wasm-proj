const thingy = document.getElementById("thingu");
const buttons = document.getElementById("buttons");

const sendMsg = (socket, event, payload) => {
    socket.send(JSON.stringify({
        topic: "room:lobby",
        event: event,
        payload: payload,
        ref: 0
    }));
}

for (i = 0; i < 4; i++) {
    const clientUUID = crypto.getRandomValues(new Uint32Array(1))[0];
    const b = document.createElement("button");
    b.innerText = `Player ${clientUUID}`;

    buttons.append(b);

    let socket = new WebSocket("ws://localhost:4000/socket/websocket");
    socket.onopen = (e) => {
        console.log(`${clientUUID} Joined :)`);
        sendMsg(socket, "phx_join", { uuid: clientUUID });
    }

    socket.onmessage = (e) => {
        const msg = JSON.parse(e.data);
        const payload = msg.payload;

        console.log(payload);
    }

    b.onclick = (e) => {
        console.log(b.innerText);
        sendMsg(socket, "next_turn", { uuid: clientUUID });
    }
}