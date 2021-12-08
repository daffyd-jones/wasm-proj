mod utils;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
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
#[derive(Clone,  Debug, PartialEq, Eq, Serialize, Deserialize,)]
pub struct Bomb {
    index: u32,
    explode_status : u32,
    x : u32,
    y : u32,
    bomb_tile : Vec<u32>
}
#[wasm_bindgen]
impl Bomb {
    pub fn new( x: u32, y:u32, width:u32, status : u32) -> Bomb {
        let index = x * width + y;
        let mut b_t = Vec::new();
        b_t.push( index +1);
        b_t.push( index -1);
        b_t.push( index + width);
        b_t.push( index - width);

        Bomb{
            index: index,
            explode_status : status,
            x: x,
            y: y,
            bomb_tile : b_t
        }
    }

}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
pub struct UniverseBombs {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    bombs: Vec<Bomb>,
    bombs_tiles : Vec<u32>
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

// #[wasm_bindgen]
// impl Universe {

//     pub fn up_move(&mut self) {
//         self.tick(InputType::Up)
//         //move player up
//     }

//     pub fn left_move(&mut self) {
//         self.tick(InputType::Left)
//         //left move
//     }

//     pub fn right_move(&mut self) {
//         self.tick(InputType::Right)
//         //right move
//     }

//     pub fn down_move(&mut self) {
//         self.tick(InputType::Down)
//         //down move
//     }

//     pub fn bomb_move(&mut self) {
//         self.tick(InputType::Bomb)
//         //down move
//     }
// }

// impl Universe {
//     // ...

//     fn neighbors(&self, row: u32, column: u32) -> Vec<Cell> {
// 		let cels = self.cells.clone();
// 		let mut adj_sqrs: Vec<Cell> = Vec::new();
// 		let idx_u = self.get_index((row - 1) % self.height, column);
// 		adj_sqrs.push(cels[idx_u]);
// 		let idx_l = self.get_index(row, (column - 1) % self.width);
// 		adj_sqrs.push(cels[idx_l]);
// 		let idx_r = self.get_index(row, (column + 1) % self.width);
// 		adj_sqrs.push(cels[idx_r]);
// 		let idx_d = self.get_index((row + 1) % self.height, column);
// 		adj_sqrs.push(cels[idx_d]);
// 		adj_sqrs
// 	}
// }

// #[wasm_bindgen]
// impl Universe {
//     pub fn tick(&mut self, input: InputType) {
//         let mut next = self.cells.clone();

//         for row in 0..self.height {
//             for col in 0..self.width {
//                 let idx = self.get_index(row, col);
//                 let cell = self.cells[idx];
//                 let mut adj_sqrs: Vec<Cell> = self.neighbors(row, col); // < --- self.neighbors
// 				let u = adj_sqrs.pop().unwrap();
// 				let l = adj_sqrs.pop().unwrap();
// 				let r = adj_sqrs.pop().unwrap();
// 				let d = adj_sqrs.pop().unwrap();
//                 // let live_neighbors = self.live_neighbor_count(row, col);

//                 let next_cell = match (u, l, r, d, input, cell) {
//                     (Cell::Player, _, _, _, InputType::Up, _) => Cell::Player,
//                     (_, Cell::Player, _, _, InputType::Left, _) => Cell::Player,
//                     (_, _, Cell::Player, _, InputType::Right, _) => Cell::Player,
//                     (_, _, _, Cell::Player, InputType::Down, _) => Cell::Player,
//                     (_, _, _, _, _, Cell::Player) => Cell::Empty,
//                     (_, _, _, _, _, cell) => cell,
//                 };

//                 next[idx] = next_cell;
//             }
//         }

//         self.cells = next;
//     }

    // ...
//}

#[wasm_bindgen]
impl UniverseBombs {
    pub fn tick(&mut self) {
        

        let mut next_bombs = Vec::new();

        let bombs = self.bombs.clone();


        
        let mut next_bombs_tiles = Vec::new();
        for mut bomb in bombs{
            let z = bomb.explode_status.clone();
            if bomb.explode_status == 1{
                bomb.explode_status = 0;
                next_bombs_tiles.append(&mut bomb.bomb_tile); 
                next_bombs.push(bomb);
            }
            else if bomb.explode_status >=2 {
                bomb.explode_status = z-1;
                next_bombs.push(bomb);

            }

        }

        let mut next = self.cells.clone();


        self.bombs_tiles = next_bombs_tiles;
        self.cells = next;
        self.bombs = next_bombs;
    }

    pub fn new() -> UniverseBombs {
        let width = 17;
        let height = 17;

        let cells = (0..width * height)
            .map(|i| {Cell::Dead
            })
            .collect();

        let mut bombs = Vec::new();

        let bomb1 = Bomb::new(3, 10, 17, 8);
        let bomb2 = Bomb::new(8, 14, 17, 5);
        let bomb3 = Bomb::new(11, 3, 17, 2);

        //bombs.push(rng.gen_range(0..width * height));
        bombs.push(bomb1);
        bombs.push(bomb2);
        bombs.push(bomb3);

        let bombs_tiles = Vec::new();

        UniverseBombs {
            width,
            height,
            cells,
            bombs,
            bombs_tiles
        }
    }

    // pub fn render(&self) -> String {
    //     self.to_string()
    // }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }


    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn bombs(&self) -> String{
        let serialized = serde_json::to_string(&self.bombs).unwrap();
        serialized
    }

    pub fn bombs_tiles(&self) -> String{
        let serialized = serde_json::to_string(&self.bombs_tiles).unwrap();
        serialized
    }
}




// use std::fmt;

// impl fmt::Display for Universe {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell == Cell::Empty { '◻' } else { '◼' };
//                 write!(f, "{}", symbol)?;
//             }
//             write!(f, "\n")?;
//         }

//         Ok(())
//     }
// }

// #[wasm_bindgen]
// impl Universe {
//     // ...

//     pub fn new() -> Universe {
//         let width = 32;
//         let height = 32;

//         let cells = (0..width * height)
//             .map(|i| {
//                 if i == 528 {
//                     Cell::Player
//                 } else if i == 2048 {
//                     Cell::Block
//                 } else {
//                     Cell::Empty
//                 }
//             })
//             .collect();

//         Universe {
//             width,
//             height,
//             cells,
//         }
//     }

//     pub fn render(&self) -> String {
//         self.to_string()
//     }
// }


