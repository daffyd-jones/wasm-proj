mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    White = 0, // Dead
    Black = 1, // Alive
	AntUpB = 2,
	AntDwnB = 3,
	AntLftB = 4,
	AntRgtB = 5,
	AntUpW = 6,
    AntDwnW = 7,
    AntLftW = 8,
    AntRgtW = 9,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    // ...

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}



impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // ...
}

impl Universe {
    // ...

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

		fn neighbors(&self, row: u32, column: u32) -> Vec<Cell> {
			let cels = self.cells.clone();
			let mut adj_sqrs: Vec<Cell> = Vec::new();
			let idx_u = self.get_index((row - 1) % self.height, column);
			adj_sqrs.push(cels[idx_u]);
			let idx_l = self.get_index(row, (column - 1) % self.width);
			adj_sqrs.push(cels[idx_l]);
			let idx_r = self.get_index(row, (column + 1) % self.width);
			adj_sqrs.push(cels[idx_r]);
			let idx_d = self.get_index((row + 1) % self.height, column);
			adj_sqrs.push(cels[idx_d]);
			adj_sqrs
		}
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
								let mut adj_sqrs: Vec<Cell> = self.neighbors(row, col); // < --- self.neighbors
								let u = adj_sqrs.pop().unwrap();
								let l = adj_sqrs.pop().unwrap();
								let r = adj_sqrs.pop().unwrap();
								let d = adj_sqrs.pop().unwrap();
								// u, l, r, d -> Cells adjacent to current cell
								// function self.neighbors returns array of u, l, r, d, cells
								// below should pattern match current cell with adjacent,
								// checking whether ant is facing this cell, current cell colour is preserved with B or W
								// if last pass had ant this pass should be opposite colour 
								// last match is (otherwise, _, _, _, _) => otherwise,

                //let live_neighbors = self.live_neighbor_count(row, col); <--- original code
                let next_cell = match (cell, u, l, r, d) {
										(Cell::White, u, _, _, _) 
											if u == Cell::AntDwnW || u == Cell::AntDwnB 
											=> Cell::AntRgtW,
										(Cell::Black, u, _, _, _) 
											if u == Cell::AntDwnW || u == Cell::AntDwnB 
											=> Cell::AntLftB,
										(Cell::White, _, l, _, _) 
											if l == Cell::AntRgtW || l == Cell::AntRgtB 
											=> Cell::AntUpW,
										(Cell::Black, _, l, _, _) 
											if l == Cell::AntRgtW || l == Cell::AntRgtB 
											=> Cell::AntDwnB,
										(Cell::White, _, _, r, _) 
											if r == Cell::AntLftW || r == Cell::AntLftB 
											=> Cell::AntDwnW,
										(Cell::Black, _, _, r, _) 
											if r == Cell::AntLftW || r == Cell::AntLftB 
											=> Cell::AntUpB,
										(Cell::White, _, _, _, d) 
											if d == Cell::AntUpW || d == Cell::AntUpB 
											=> Cell::AntLftW,
										(Cell::Black, _, _, _, d) 
											if d == Cell::AntUpW || d == Cell::AntUpB 
											=> Cell::AntRgtB,
										
										(Cell::AntRgtW, _, _, _, _) => Cell::Black,
                    (Cell::AntLftW, _, _, _, _) => Cell::Black,
                    (Cell::AntUpW, _, _, _, _) => Cell::Black,
                    (Cell::AntDwnW, _, _, _, _) => Cell::Black,
										(Cell::AntRgtB, _, _, _, _) => Cell::White,
                    (Cell::AntLftB, _, _, _, _) => Cell::White,
                    (Cell::AntUpB, _, _, _, _) => Cell::White,
                    (Cell::AntDwnB, _, _, _, _) => Cell::White,
                    //(Cell::Black, x) if x < 2 => Cell::White,
                    //(Cell::Black, 2) | (Cell::Black, 3) => Cell::Black,
                    //(Cell::Black, x) if x > 3 => Cell::White,
                    //(Cell::White, 3) => Cell::Black,
                    (otherwise, _, _, _, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    // ...
}


use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::White { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    // ...

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 53 == 0 || i % 71 == 0 {
										Cell::Black
								}
								else if i % 271 == 0 { //== ((width * height) / 2) + width/2 {
                    Cell::AntUpW
                } else {
                    Cell::White
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}


