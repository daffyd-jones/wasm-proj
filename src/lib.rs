mod utils;
mod bomb;
use bomb::BombStruct;
mod player;
use player::Player;

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
    Empty = 0,
    Player = 1,
    OthrPlyr = 2,
    Block = 3,
    Bomb = 4,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BombGrid {
    Empty = 0,
    Occupied = 1
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputType {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Bomb = 4,
}

#[wasm_bindgen]
pub struct Universe {
    host: bool,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    bombs_vec: Vec<BombStruct>,
    bombs_locations: Vec<BombGrid>,
    players_vec: Vec<Player>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // ...
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

#[wasm_bindgen]
impl Universe {

    pub fn up_move(&mut self) {
        self.tick(InputType::Up)
        //move player up
    }

    pub fn left_move(&mut self) {
        self.tick(InputType::Left)
        //left move
    }

    pub fn right_move(&mut self) {
        self.tick(InputType::Right)
        //right move
    }

    pub fn down_move(&mut self) {
        self.tick(InputType::Down)
        //down move
    }

    pub fn bomb_move(&mut self) {
        self.tick(InputType::Bomb)
        //down move
    }
}

impl Universe {
    // ...

    // fn neighbors(&self, row: u32, column: u32) -> Vec<Cell> {
	// 	let cels = self.cells.clone();
	// 	let mut adj_sqrs: Vec<Cell> = Vec::new();
	// 	let idx_u = self.get_index((row - 1) % self.height, column);
	// 	adj_sqrs.push(cels[idx_u]);
	// 	let idx_l = self.get_index(row, (column - 1) % self.width);
	// 	adj_sqrs.push(cels[idx_l]);
	// 	let idx_r = self.get_index(row, (column + 1) % self.width);
	// 	adj_sqrs.push(cels[idx_r]);
	// 	let idx_d = self.get_index((row + 1) % self.height, column);
	// 	adj_sqrs.push(cels[idx_d]);
	// 	adj_sqrs
	// }
}

#[wasm_bindgen]
impl Universe {
    // TODO: where do we create a new player?
    pub fn tick(&mut self, input: InputType) {
        // let mut next = self.cells.clone();

        // for row in 0..self.height {
        //     for col in 0..self.width {
        //         let idx = self.get_index(row, col);
        //         let cell = self.cells[idx];
        //         let mut adj_sqrs: Vec<Cell> = self.neighbors(row, col); // < --- self.neighbors
		// 		let u = adj_sqrs.pop().unwrap();
		// 		let l = adj_sqrs.pop().unwrap();
		// 		let r = adj_sqrs.pop().unwrap();
		// 		let d = adj_sqrs.pop().unwrap();
        //         // let live_neighbors = self.live_neighbor_count(row, col);

        //         let next_cell = match (u, l, r, d, input, cell) {
        //             (Cell::Player, _, _, _, InputType::Up, _) => Cell::Player,
        //             (_, Cell::Player, _, _, InputType::Left, _) => Cell::Player,
        //             (_, _, Cell::Player, _, InputType::Right, _) => Cell::Player,
        //             (_, _, _, Cell::Player, InputType::Down, _) => Cell::Player,
        //             (_, _, _, _, _, Cell::Player) => Cell::Empty,
        //             (_, _, _, _, _, cell) => cell,
        //         };

        //         next[idx] = next_cell;
        //     }
        // }

        // self.cells = next;

        let mut plyrs = self.players_vec.clone();

        for p in plyrs.iter_mut() {
            if p.host() == self.host {
                match input {
                    InputType::Up => p.up(),
                    InputType::Left => p.left(),
                    InputType::Right => p.right(),
                    InputType::Down => p.down(),
                    InputType::Bomb => p.drop_bomb(),
                }
            }
        }

        self.players_vec = plyrs;

        let mut plyrs = self.players_vec.clone();

        // tick down bombs
        for b in self.bombs_vec.iter_mut() {
            b.count_down();
            if b.timer() == 0 {
                let affected_tiles = b.explosion_tiles();
                // TODO: check players positions and call their lose_hp() if they're hit
                for (x, y) in affected_tiles.iter() {
                    // self.players_vec = self.players_vec.iter()
                    //     .map(|&i| if i.space(x, y) {i.lose_hp()}).collect();
                    for i in 0..plyrs.len() {
                        let px = &plyrs[i].x();
                        let py = &plyrs[i].y();
                        if (px, py) == (x, y) {
                            plyrs[i].lose_hp();
                        }
                    }
                } 
                
                // TODO: check walls (julie working on this?)
            }
        }
        
        self.players_vec = plyrs;

        // // if input type is bomb
        // if input == InputType::Bomb {
        //     // TODO: get player id and position to create new bomb and push to bombs_vec
        // }


        
    }

    // ...
}


use std::fmt;
use crate::InputType::Bomb;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Empty { '◻' } else { '◼' };
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
        let width = 32;
        let height = 32;

        let cells = (0..width * height)
            .map(|i| {
                if i == 528 {
                    Cell::Player
                } else if i == 2048 {
                    Cell::Block
                } else {
                    Cell::Empty
                }
            })
            .collect();

        let bombs_locations = (0..width * height)
            .map(|i| {
                BombGrid::Empty
            })
            .collect();
        
        let bombs_vec: Vec<BombStruct> = Vec::new();
        let host_player = Player::new(true, 24, 24, 10);
        let guest_player = Player::new(false, 40, 40, 10);
        let mut players_vec: Vec<Player> = Vec::new(); {}
        players_vec.push(host_player);
        players_vec.push(guest_player);

        let host = true;

        Universe {
            host,
            width,
            height,
            cells,
            bombs_vec,
            bombs_locations,
            players_vec
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}


