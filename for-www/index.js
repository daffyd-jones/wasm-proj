import { Universe, Cell } from "wasm-proj";

const CELL_SIZE = 25; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const BLOCK_COLOR = "#DE3163";
const ANTD_COLOR = "#DFFF00";
const ANTL_COLOR = "#40E0D0";
const ANTR_COLOR = "#CCCCFF";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const socket = new WebSocket("ws://localhost:4000/socket/websocket");

const refMake = () => Math.floor(Math.random() * 9999999);

let joined = false;
const joinResponseRef = refMake();
let yourTurn = false;
let inspectionRef = 0;


// const renderLoop = () => {

//   drawGrid();
//   drawCells();
// };

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

import { memory } from "../pkg/wasm_proj_bg";

// ...

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  // const cellsPtr = universe.cells();
  // const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  // ctx.beginPath();

  // for (let row = 0; row < height; row++) {
  //   for (let col = 0; col < width; col++) {
  //     const idx = getIndex(row, col);

  //     // ctx.fillStyle = cells[idx] === Cell.Empty
  //     //   ? DEAD_COLOR
  //     //   : ALIVE_COLOR;

  //     switch (cells[idx]) {
  // 			case Cell.Empty:
  // 				ctx.fillStyle = DEAD_COLOR;
  // 				break;
  // 			case Cell.Player:
  // 				ctx.fillStyle = ALIVE_COLOR;
  //       	break;	
  // 			case Cell.Block:
  //         ctx.fillStyle = BLOCK_COLOR;
  //         break;
  // 			default:
  // 				ctx.fillStyle = "#FFBF00";
  // 		}

  //     ctx.fillRect(
  //       col * (CELL_SIZE + 1) + 1,
  //       row * (CELL_SIZE + 1) + 1,
  //       CELL_SIZE,
  //       CELL_SIZE
  //     );
  //   }
  // }

  // ctx.stroke();
};



function setEventListener() {
  window.addEventListener("keydown", function (event) {
    if (!yourTurn) return;

    if (event.defaultPrevented) {
      return;
    }

    switch (event.key) {
      case "ArrowDown":
        console.log("arrow down");
        const dres = universe.down_move();
        switch (dres) {
          case "fail":
            console.log("move failed, cell occupied");
            break;
          case "pass":
            console.log("move successful");
            yourTurn = false;
            break;
        }
        break;
      case "ArrowUp":
        console.log("arrow up");
        const ures = universe.up_move();
        switch (ures) {
          case "fail":
            console.log("move failed, cell occupied");
            break;
          case "pass":
            console.log("move successful");
            yourTurn = false;
            break;
        }
        break;
      case "ArrowLeft":
        console.log("arrow left");
        const lres = universe.left_move();
        switch (lres) {
          case "fail":
            console.log("move failed, cell occupied");
            break;
          case "pass":
            console.log("move successful");
            yourTurn = false;
            break;
        }
        break;
      case "ArrowRight":
        console.log("arrow right");
        const rres = universe.right_move();
        switch (rres) {
          case "fail":
            console.log("move failed, cell occupied");
            break;
          case "pass":
            console.log("move successful");
            yourTurn = false;
            break;
        }
        break;
      case "b":
        console.log("space");
        const bres = universe.bomb_move();
        switch (bres) {
          case "fail":
            console.log("move failed, cell occupied");
            break;
          case "pass":
            console.log("move successful");
            yourTurn = false;
            break;
        }
        break;
      default:
        console.log(event.key);
        return;
    }
    event.preventDefault();

    // If a move was successfully made...
    if (!yourTurn) {
      let pbw = extractPlayersBombsWalls();
      let turnMessage = PhoenixEvent("finish_turn", "room:lobby", JSON.stringify(pbw), refMake());
      socket.send(turnMessage);
    }

    const walls = universe.walls();
    const bombs = universe.bombs();
    const players = universe.players();
    console.log(walls);
    console.log(bombs);
    console.log(players);
    drawGrid();
    drawCells();
    // requestAnimationFrame(renderLoop);

    // UNCOMMENT THE FOLLOWING IN CASE OF TURN JAMMING:
    // yourTurn = true;
  }, true);
}

function PhoenixEvent(event, topic, payload, ref) {
  return {
    event: event,
    topic: topic,
    payload: payload,
    ref: ref,
  }
}

function extractPlayersBombsWalls() {
  return {
    players: JSON.parse(universe.players()),
    bombs: JSON.parse(universe.bombs()),
    walls: JSON.parse(universe.walls()),
  }
}

function insertPlayersBombsWalls(new_state) {
  universe.set_players(JSON.stringify(new_state.players));
  universe.set_bombs(JSON.stringify(new_state.bombs));
  universe.set_walls(JSON.stringify(new_state.walls));
}

function socketEvents() {
  socket.addEventListener("open", e => {
    console.log("Attempting join...");
    let message = PhoenixEvent("phx_join", "room:lobby", JSON.stringify({ uuid: universe.host_id() }), joinResponseRef);
    socket.send(message);
  });

  socket.addEventListener("message", event => {
    let data = JSON.parse(event.data);
    switch (data.event) {
      case "phx_reply":
        if (!joined && data.ref === joinResponseRef) {
          if (data.payload.status === "ok") {
            // Join confirmed.
            joined = true;
            inspectionRef = refMake();
            let stateMessage = PhoenixEvent("inspect_state", "room:lobby", {}, inspectionRef);
            socket.send(stateMessage);
          } else {
            console.error("Join failed!");
          }
        } else if (data.ref === inspectionRef) {
          if (data.payload.status === "ok") {
            insertPlayersBombsWalls(data.payload.response);
          } else {
            console.error("State retrival failed!");
          }
        }
        break;
      case "new_plr":
        let new_player = JSON.stringify(data.payload);
        universe.new_player(new_player);
        break;
      case "new_turn":
        if (data.payload.next_player === universe.host_id()) {
          yourTurn = true;
        }

        insertPlayersBombsWalls(data.payload.new_state);

        break;
    }
  });
}
setEventListener();
universe.tick();
drawGrid();
drawCells();
// requestAnimationFrame(renderLoop);