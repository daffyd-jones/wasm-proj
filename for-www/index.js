import { Universe, Cell } from "wasm-game-of-life";

const CELL_SIZE = 13; // px
const GRID_COLOR = "#CCCCCC";
const WHITE_COLOR = "#FFFFFF";
const BLACK_COLOR = "#000000";
const ANTU_COLOR = "#DE3163";
const ANTD_COLOR = "#DFFF00";
const ANTL_COLOR = "#40E0D0";
const ANTR_COLOR = "#CCCCFF";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  universe.tick();

  drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop);
};

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

import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// ...

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
    	const idx = getIndex(row, col);
			
			//let colr = "#888888";
			
			//let cell = cells[idx];
			
			switch (cells[idx]) {
				case Cell.White:
					ctx.fillStyle = WHITE_COLOR;
					break;
				case Cell.Black:
					ctx.fillStyle = BLACK_COLOR;
        	break;	
				case Cell.AntLftW:
          ctx.fillStyle = ANTL_COLOR;
          break;
        case Cell.AntLftB:
          ctx.fillStyle = ANTL_COLOR;
          break;
				case Cell.AntRgtW:
          ctx.fillStyle = ANTR_COLOR;
          break;
        case Cell.AntRgtB:
          ctx.fillStyle = ANTR_COLOR;
          break;
				case Cell.AntUpW:
          ctx.fillStyle = ANTU_COLOR;
          break;
        case Cell.AntUpB:
          ctx.fillStyle = ANTU_COLOR;
          break;
		 		case Cell.AntDwnW:
          ctx.fillStyle = ANTD_COLOR;
          break;
        case Cell.AntDwnB:
          ctx.fillStyle = ANTD_COLOR;
          break;
				default:
					ctx.fillStyle = "#FFBF00";
			}
			
			//if cell === Cell.White {
      //		colr = WHITE_COLOR;
      //  } else if cell === Cell.Black {
      //    colr = BLACK_COLOR;
      //  } else {
      //    colr = ANTD_COLOR; // <- this would be a switch or a longer else if
      //  }

      //ctx.fillStyle = cells[idx] === Cell.White
			//	? WHITE_COLOR
			//	: BLACK_COLOR;
			
			//ctx.fillstyle = colr;
			
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
