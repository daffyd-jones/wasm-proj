mod utils;

mod bomb;
use bomb::BombStruct;
mod wall;
use wall::WallStruct;
mod player;
use player::Player;

use rand::Rng;

use serde::{Deserialize, Serialize};
use serde_json::Result;

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
    host_id: i32,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    bombs_vec: Vec<BombStruct>,
    bombs_locations: Vec<BombGrid>,
    players_vec: Vec<Player>,
    walls_vec: Vec<WallStruct>
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

    pub fn up_move(&mut self) -> String {
        return self.tick(InputType::Up);
        //move player up
    }

    pub fn left_move(&mut self) -> String {
        return self.tick(InputType::Left);
        //left move
    }

    pub fn right_move(&mut self) -> String {
        return self.tick(InputType::Right);
        //right move
    }

    pub fn down_move(&mut self) -> String {
        return self.tick(InputType::Down);
        //down move
    }

    pub fn bomb_move(&mut self) -> String {
        return self.tick(InputType::Bomb);
        //down move
    }

    pub fn bombs(&self) -> String {
        let serialized = serde_json::to_string(&self.bombs_vec).unwrap();
        serialized
    }

    pub fn walls(&self) -> String {
        let serialized = serde_json::to_string(&self.walls_vec).unwrap();
        serialized
    }

    pub fn players(&self) -> String {
        let serialized = serde_json::to_string(&self.players_vec).unwrap();
        serialized
    }

    pub fn set_players(&mut self, players: String) {
        let deserialized: Vec<Player> = serde_json::from_str(&players).unwrap();
        self.players_vec = deserialized;
    }

    pub fn set_walls(&mut self, walls: String) {
        let deserialized: Vec<WallStruct> = serde_json::from_str(&walls).unwrap();
        self.walls_vec = deserialized;
    }

    pub fn set_bombs(&mut self, bombs: String) {
        let deserialized: Vec<BombStruct> = serde_json::from_str(&bombs).unwrap();
        self.bombs_vec = deserialized;
    }


}

impl Universe {
    // ...


    fn occupied(&self, col: i32, row: i32) -> bool {
        let mut bomb_check = false;
        let mut wall_check = false;
        let mut player_check = false;

        let bombs = self.bombs_vec.clone();
        let walls = self.walls_vec.clone();
        let players = self.players_vec.clone();

        for w in walls.iter() {
            if w.x() == row && w.y() == col {
                wall_check = true;
            }
        }

        for b in bombs.iter() {
            if b.x() == row && b.y() == col {
                bomb_check = true;
            }
        }

        for p in players.iter() {
            if p.x() == row && p.y() == col {
                player_check = true;
            }
        }
        
        return wall_check || bomb_check || player_check;
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

    fn bomb_here(&self, x: i32, y: i32) -> bool {
        let bombs = self.bombs_vec.clone();
        for b in bombs.iter() {
            if b.x() == x && b.y() == y {
                return true;
            }
        }
        return false;
    }
 
    fn place_bomb(&mut self) -> bool {
        let mut bombs = self.bombs_vec.clone();
        let mut players = self.players_vec.clone();
        for p in players.iter_mut() {
            if p.id() == self.host_id {
                if self.bomb_here(p.x(), p.y()) {
                    return true;
                }
                let bomb = BombStruct::new(p.x(), p.y());
                bombs.push(bomb);
                p.drop_bomb();
            }
        }
        self.players_vec = players;
        self.bombs_vec = bombs;
        return false;
    }
}

#[wasm_bindgen]
impl Universe {
    // TODO: where do we create a new player?
    pub fn tick(&mut self, input: InputType) -> String {
        let mut plyrs = self.players_vec.clone();
        let mut fail = false;

        for p in plyrs.iter_mut() {
            if p.id() == self.host_id {
                match input {
                    InputType::Up if !self.occupied(p.x(), p.y() - 1) => p.up(),
                    InputType::Left if !self.occupied(p.x() - 1, p.y()) => p.left(),
                    InputType::Right if !self.occupied(p.x() + 1, p.y()) => p.right(),
                    InputType::Down if !self.occupied(p.x(), p.y() + 1) => p.down(),
                    InputType::Bomb => fail = self.place_bomb(),
                    _ => fail = true
                }
            }
        }

        if fail {
            return String::from("fail");
        }

        self.players_vec = plyrs;

        let mut plyrs = self.players_vec.clone();
        let mut walls = self.walls_vec.clone();
        let mut bombs = self.bombs_vec.clone();

        let mut dead_bombs: Vec<usize> = Vec::new();
        let mut idx = 0;
        // tick down bombs
        for b in bombs.iter_mut() {
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
                    // TODO: check walls (julie working on this?)
                    for j in 0..walls.len(){
                        let wx = &walls[j].x();
                        let wy = &walls[j].y();
                        if (wx, wy) == (x , y) {
                            walls[j].is_bombed();
                        }
                    }
                } 

                dead_bombs.push(idx);
            }
            idx += 1;
        }

        for i in dead_bombs.iter() {
            bombs.remove(*i);
        }
        self.bombs_vec = bombs;
        self.players_vec = plyrs;
        self.walls_vec = walls;

        return String::from("pass"); 
    }

    // ...
}


use std::fmt;
// use crate::InputType::Bomb;

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
        let width = 33;
        let height = 33;

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

        let mut rng = rand::thread_rng();
        let num = rng.gen::<i32>();
        let host_player = Player::new(num, 1, 1, 10);
        let guest_player = Player::new(2, 31, 31, 10);
        let mut players_vec: Vec<Player> = Vec::new(); {}
        players_vec.push(host_player);
        players_vec.push(guest_player);

        // Construct the solid walls for the launch of universe
        let mut walls_vec: Vec<WallStruct> = Vec::new();

        // let wid = width.clone();
        // let hi = height.clone();

        for i in 0..width {
            for j in 0.. height{
                // add indestructible walls
                if i == 0 || j == 0 || i == width - 1 || j == height - 1 {
                    walls_vec.push(WallStruct::new(i, j, false, true))
                }
                else if (i % 2 == 0) && (j % 2 == 0) {
                    // leave space around players with no walls
                    if (i == 2 && (j == 2 || j == height - 3)) || (i == width - 3 && (j == 2 || j == height - 3)) {
                        continue;
                    } else {
                        walls_vec.push(WallStruct::new(i, j, false, true))
                    }
                }
                // add indestructible walls
                else if i != 1 && i != width - 2 && j != 1 && j != height - 2 {
                    let rand_no_wall_i = rng.gen_range(2..width - 2);
                    let rand_no_wall_j = rng.gen_range(2..height - 2);
                    if i != rand_no_wall_i && j != rand_no_wall_j {
                        walls_vec.push(WallStruct::new(i, j, true, true))
                    }
                }
            }
        }

        Universe {
            host_id: num,
            width: 33,
            height: 33,
            cells,
            bombs_vec,
            bombs_locations,
            players_vec,
            walls_vec
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}


