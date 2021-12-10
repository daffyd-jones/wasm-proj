/*
 scuffed as heck code to test out the server code
 */

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

let socket = new WebSocket("ws://localhost:4000/socket/websocket");
const clientUUID = crypto.getRandomValues(new Uint32Array(1))[0];

let players = [];
let clientPlayer;
const keyboard = {
    left: false,
    up: false,
    right: false,
    down: false,
};

const sendMsg = (event, payload) => {
    socket.send(JSON.stringify({
        topic: "room:lobby",
        event: event,
        payload: payload,
        ref: 0
    }));
}

socket.onopen = (e) => {
    console.log(e);

    sendMsg("phx_join", { uuid: clientUUID });
};

socket.onmessage = (e) => {
    const msg = JSON.parse(e.data);
    const payload = msg.payload;

    switch (msg.event) {
        case "new_plr":
            const newPlayer = payload;
            
            // bruh
            if (newPlayer.id === clientUUID) {
                sendMsg("inspect_all", {});
            } else {
                players.push(newPlayer);
            }
            break;
        case "new_pos":
            if (payload.id === clientUUID) return;
            const player = findPlayer(payload.id);
            player.x = payload.x;
            player.y = payload.y;
            break;
        case "phx_reply":
            // ;_; would be nice if javascript had pattern matching
            if (payload.response && Array.isArray(payload.response)) {
                players = payload.response;
                clientPlayer = findPlayer(clientUUID);
                requestAnimationFrame(draw);
            }
            break;
    }
}

socket.onclose = (e) => {
    console.log(`Closed: code=${e.code}, reason=${e.reason}`);
}

socket.onerror = (err) => {
    console.log(err.message);
}

const drawSquare = (x, y, sl) => {
    ctx.fillStyle = "#666666";
    ctx.fillRect(x, y, sl, sl);
}

window.onkeydown = (e) => {
    const code = e.keyCode;
    switch(code) {
        case 37:
            keyboard.left = true;
            break;
        case 38:
            keyboard.up = true;
            break;
        case 39:
            keyboard.right = true;
            break;
        case 40:
            keyboard.down = true;
            break;
    }
}

window.onkeyup = (e) => {
    const code = e.keyCode;
    switch(code) {
        case 37:
            keyboard.left = false;
            break;
        case 38:
            keyboard.up = false;
            break;
        case 39:
            keyboard.right = false;
            break;
        case 40:
            keyboard.down = false;
            break;
    }
}

const findPlayer = (uuid) => {
    return players.find((plr) => plr.id === uuid);
}

const draw = () => {
    ctx.clearRect(0, 0, 1000, 1000);

    updatePlr();

    for (let i = 0; i < players.length; i++) {
        const player = players[i];
        drawSquare(player.x, player.y, 40);
    }

    requestAnimationFrame(draw);
}

const moveRate = 3;
const updatePlr = () => {
    if (keyboard.left) clientPlayer.x -= moveRate;
    else if (keyboard.right) clientPlayer.x += moveRate;
    if (keyboard.up) clientPlayer.y -= moveRate;
    else if (keyboard.down) clientPlayer.y += moveRate;

    sendMsg("update_pos", {
        new_pos: {
            x: clientPlayer.x,
            y: clientPlayer.y,
        },
    });
}