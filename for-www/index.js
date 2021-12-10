import { Universe, Cell } from "wasm-proj";
import { memory } from "../pkg/wasm_proj_bg";

const CELL_SIZE = 25; // px
const GRID_COLOR = "#CCCCCC";
const GRID_FILL = "2e7700";

// Load images
let playerImgOne = new Image();
playerImgOne.src = "./images/player1.png";
let playerImgTwo = new Image();
playerImgTwo.src = "./images/player2.png";
const bombOne = new Image();
bombOne.src = "./images/bomb1.png";
const bombTwo = new Image();
bombTwo.src = "./images/bomb2.png";
const bombThree = new Image();
bombThree.src = "./images/bomb3.png";
const bombFour = new Image();
bombFour.src = "./images/bomb4.png";
const bombFive = new Image();
bombFive.src = "./images/bomb5.png";
const wallImgDes = new Image();
wallImgDes.src = "./images/wall-destructable.png";
const wallImgSolid = new Image();
wallImgSolid.onload = start;
wallImgSolid.src = "./images/wall-solid.png";
const walkSound = new sound("sounds/walk.wav");
const bombSound = new sound("sounds/bomb.wav");
const bPlant = new sound("sounds/plant.wav");
const background = new sound("sounds/background.mp3");
let wasOne = false;

let win = false;
let lose = false;

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Get initial walls, players, and bombs
let walls = JSON.parse(universe.walls());
let players = JSON.parse(universe.players());
let bombs = JSON.parse(universe.bombs());
console.log(universe.explosions());

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

const socket = new WebSocket("ws://localhost:4000/socket/websocket");

const refMake = () => Math.floor(Math.random() * 9999999);

let joined = false;
const joinResponseRef = refMake();
let yourTurn = false;
let inspectionRef = 0;

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

const getIndex = (row, column) => {
  return row * width + column;
};

function PhoenixEvent(event, topic, payload, ref) {
  return JSON.stringify({
    event: event,
    topic: topic,
    payload: payload,
    ref: ref,
  });
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
    let message = PhoenixEvent(
      "phx_join", 
      "room:lobby", 
      { uuid: universe.host_id() }, 
      joinResponseRef
    );
    socket.send(message);
  });

  socket.addEventListener("message", event => {
    let data = JSON.parse(event.data);

    console.log(data);
    switch (data.event) {
      case "phx_reply":
        if (!joined && data.ref === joinResponseRef) {
          if (data.payload.status === "ok") {
            // Join confirmed.
            joined = true;
          } else {
            console.error("Join failed!");
          }
        //} else if (data.ref === inspectionRef) {
        } else if (data.ref === inspectionRef) {
          const payload = data.payload;
          // Response for "inspect_state"
          if (payload.status === "ok" && payload.response.players) {
            const state = data.payload.response;
            console.log("State retrieved");
            
            if (state.players.length !== 1) {
              insertPlayersBombsWalls(state);
              
              walls = JSON.parse(universe.walls());
              players = JSON.parse(universe.players());
              bombs = JSON.parse(universe.bombs());

              clearGrid();
              drawGrid();
              drawWalls(walls);
              drawPlayers(players);
              drawBombs(bombs);
            } else {
              yourTurn = true;
              // Dumb hack so the first client saves the state to the server
              // first
              const pbw = extractPlayersBombsWalls();
              const turnMessage = PhoenixEvent(
                "finish_turn", 
                "room:lobby", 
                pbw, 
                refMake()
              );
              socket.send(turnMessage);
            }
          }
        }

        break;
      case "new_plr":
        // If the new player happens to be the current client
        if (data.payload.id == universe.host_id()) {
          const stateMessage = PhoenixEvent(
            "inspect_state", 
            "room:lobby", 
            {}, 
            refMake()
          );
          socket.send(stateMessage);
          break; 
        }

        let new_player = JSON.stringify(data.payload);
        universe.add_player(new_player);

        players = JSON.parse(universe.players());

        clearGrid();
        drawGrid();
        drawWalls(walls);
        drawPlayers(players);
        drawBombs(bombs);
        break;
      case "new_turn":
        console.log(`It is ${data.payload.next_player}'s turn`);

        if (data.payload.next_player === universe.host_id()) {
          yourTurn = true;
        }

        insertPlayersBombsWalls(data.payload.new_state);
        
        walls = JSON.parse(universe.walls());
        bombs = JSON.parse(universe.bombs());
        players = JSON.parse(universe.players());
        
        clearGrid();
        drawGrid();
        drawWalls(walls);
        drawPlayers(players);
        drawBombs(bombs);
        break;
    }
  });
}

const clearGrid = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
};

const drawWalls = (walls) => {
  walls.forEach((wall) => {
    if (wall.alive) {
      let row = wall.x;
      let col = wall.y;

      if (wall.destructible) {
        ctx.drawImage(
          wallImgDes,
          row * (CELL_SIZE + 1) + 1,
          col * (CELL_SIZE + 1) + 1
        );
        // console.log("breakable");
        // console.log(row + ", " + col);
      } else {
        ctx.drawImage(
          wallImgSolid,
          row * (CELL_SIZE + 1) + 1,
          col * (CELL_SIZE + 1) + 1
        );
        // console.log("solid");
        // console.log(row + ", " + col);
      }
    } else {
      let row = wall.x;
      let col = wall.y;
      console.log("dead");
      console.log(row + ", " + col);
    }
  });
};

const drawPlayers = (players) => {
  players.forEach((player) => {
    if (player.alive) {
      if (player.id == universe.host_id()) {
        ctx.drawImage(
          playerImgOne,
          player.x * (CELL_SIZE + 1) + 1,
          player.y * (CELL_SIZE + 1) + 1
        );
      } else {
        ctx.drawImage(
          playerImgTwo,
          player.x * (CELL_SIZE + 1) + 1,
          player.y * (CELL_SIZE + 1) + 1
        );
      }
    }
  });
};

const drawBombs = (bombs) => {
  if (bombs.length == 0 && wasOne == true) {
    bombSound.play();
    wasOne = false;
  }
  bombs.forEach((bomb) => {
    console.log(bomb.timer)
    if (Math.ceil(bomb.timer/2) == 1) {
      wasOne = true;
      ctx.drawImage(
        bombOne,
        bomb.x * (CELL_SIZE + 1) + 1,
        bomb.y * (CELL_SIZE + 1) + 1
      );
    } else if (Math.ceil(bomb.timer/2) == 2) {
      ctx.drawImage(
        bombTwo,
        bomb.x * (CELL_SIZE + 1) + 1,
        bomb.y * (CELL_SIZE + 1) + 1
      );
    } else if (Math.ceil(bomb.timer/2) == 3) {
      ctx.drawImage(
        bombThree,
        bomb.x * (CELL_SIZE + 1) + 1,
        bomb.y * (CELL_SIZE + 1) + 1
      );
    } else if (Math.ceil(bomb.timer/2) == 4) {
      ctx.drawImage(
        bombFour,
        bomb.x * (CELL_SIZE + 1) + 1,
        bomb.y * (CELL_SIZE + 1) + 1
      );
    } else if (Math.ceil(bomb.timer/2) == 5) {
      ctx.drawImage(
        bombFive,
        bomb.x * (CELL_SIZE + 1) + 1,
        bomb.y * (CELL_SIZE + 1) + 1
      );
    }
  });
};
// console.log(check);

function sound(src) {
  this.sound = document.createElement("audio");
  this.sound.src = src;
  this.sound.setAttribute("preload", "auto");
  this.sound.setAttribute("controls", "none");
  this.sound.style.display = "none";
  document.body.appendChild(this.sound);
  this.play = function () {
    this.sound.play();
  }
  this.stop = function () {
    this.sound.pause();
  }
  this.loop = function () {
    this.sound.setAttribute("volume", "0.5");
    this.sound.play();
    this.sound.loop = true;
  }
}

function setEventListener() {
  window.addEventListener(
    "keydown",
    function (event) {
      background.loop();
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
              walkSound.play();
              break;
            case "lose":
              console.log("you lose");
              const playerImgOne = new Image();
              playerImgOne.src = "./images/player1-dead.png";
              lose = true;
              break;
            case "win":
              console.log("you win");
              const playerImgTwo = new Image();
              playerImgTwo.src = "./images/player2-dead.png";
              win = true;
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
              walkSound.play();
              break;
            case "lose":
              console.log("you lose");
              const playerImgOne = new Image();
              playerImgOne.src = "./images/player1-dead.png";
              lose = true;
              break;
            case "win":
              console.log("you win");
              const playerImgTwo = new Image();
              playerImgTwo.src = "./images/player2-dead.png";
              win = true;
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
              walkSound.play();
              yourTurn = false;
              break;
            case "lose":
              console.log("you lose");
              const playerImgOne = new Image();
              playerImgOne.src = "./images/player1-dead.png";
              lose = true;
              break;
            case "win":
              console.log("you win");
              const playerImgTwo = new Image();
              playerImgTwo.src = "./images/player2-dead.png";
              win = true;
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
              walkSound.play();
              yourTurn = false;
              break;
            case "lose":
              console.log("you lose");
              const playerImgOne = new Image();
              playerImgOne.src = "./images/player1-dead.png";
              lose = true;
              break;
            case "win":
              console.log("you win");
              const playerImgTwo = new Image();
              playerImgTwo.src = "./images/player2-dead.png";
              win = true;
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
              bPlant.play();
              console.log("move successful");
              break;
            case "lose":
              console.log("you lose");
              const playerImgOne = new Image();
              playerImgOne.src = "./images/player1-dead.png";
              lose = true;
              break;
            case "win":
              console.log("you win");
              const playerImgTwo = new Image();
              playerImgTwo.src = "./images/player2-dead.png";
              win = true;
              break;
          }
          break;
        default:
          console.log(event.key);
          return;
      }
      event.preventDefault();

      console.log(win);
      console.log(lose);
      
      // Successful turn
      if (!yourTurn) {
        let pbw = extractPlayersBombsWalls();
        let turnMessage = PhoenixEvent(
          "finish_turn", 
          "room:lobby", 
          pbw, 
          refMake()
        );
        socket.send(turnMessage);
      }

      walls = JSON.parse(universe.walls());
      bombs = JSON.parse(universe.bombs());
      players = JSON.parse(universe.players());
      // console.log(walls);
      // let occupied = JSON.parse(universe.occupy());
      // console.log(occupied);
      // let explosions = JSON.parse(universe.explosions());
      // console.log(explosions);
      clearGrid();
      drawGrid();
      drawWalls(walls);
      drawPlayers(players);
      drawBombs(bombs);
    },
    true
  );
}

function start() {
  setEventListener();
  universe.tick();
  socketEvents();
  drawGrid();
  drawWalls(walls);
  drawPlayers(players);
}
