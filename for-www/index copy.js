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
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

import { memory } from "wasm-proj/wasm_proj_bg";

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
            break;
          case "lose":
            console.log("you lose");
            break;
          case "win":
            console.log("you win");
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
            break;
          case "lose":
            console.log("you lose");
            break;
          case "win":
            console.log("you win");
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
            break;
          case "lose":
            console.log("you lose");
            break;
          case "win":
            console.log("you win");
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
            break;
          case "lose":
            console.log("you lose");
            break;
          case "win":
            console.log("you win");
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
            break;
          case "lose":
            console.log("you lose");
            break;
          case "win":
            console.log("you win");
            break;
        }
        break;
      default:
        console.log(event.key);
        return; 
    }
    event.preventDefault();
    const walls = universe.walls();
    const bombs = universe.bombs();
    const players = universe.players();
    console.log(walls);
    console.log(bombs);
    console.log(players);
    drawGrid();
    drawCells();
    // requestAnimationFrame(renderLoop);
  }, true);
}
setEventListener();
universe.tick();
drawGrid();
drawCells();
// requestAnimationFrame(renderLoop);