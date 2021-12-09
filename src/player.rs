// mod bomb;
// use bomb::Bomb;
use serde::{Serialize, Deserialize};

// Struct for Players
  // name: name of player character
  // id: id
  // x: x coord
  // y: y coord
  // bombs: number of bombs
  // power: power level of bombs (how many tiles from center it explodes)
  // alive: is player alive
  // hp: how many lives a player has left
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Player {
  // name: String,
  id: i32,
  x: i32, 
  y: i32,
  bombs: i32,
  alive: bool,
  hp: i32,
}

impl Player {
  
  // constructor for new players
  pub fn new(id: i32, x: i32, y: i32, bombs: i32) -> Player {
    Player {id: id, x: x, y: y, bombs: bombs, alive: true, hp: 5}
  }

  // move character to specified coordinates.
  pub fn move_to(&mut self, x: i32, y:i32) {
    self.x = x;
    self.y = y;
  }

  // increment position of player
  // TODO: check to make sure within bounds of play
  pub fn up(&mut self) {
    self.move_to(self.x, self.y - 1)
  }

  pub fn down(&mut self) {
    self.move_to(self.x, self.y + 1)
  }

  pub fn left(&mut self) {
    self.move_to(self.x - 1, self.y)
  }

  pub fn right(&mut self) {
    self.move_to(self.x + 1, self.y)
  }

  // lose life
  pub fn lose_hp(&mut self) {
    self.hp -= 1;
    if self.hp == 0 {
      self.alive = false;
    }
  }

  // unalive self.
  fn kill(&mut self) {
    self.alive = false;
  }

  // TODO: original drop_bomb func created a cycle import issue with calling mod bomb on player and lib
  // get a bomb placed at current location.
  // pub fn drop_bomb(&mut self) -> Option<Bomb> {
  //   if self.bombs > 0 {
  //     self.bombs -= 1;
  //     Some(Bomb.new(self.x, self.y))
  //   }
  //   None
  // }
  // TODO: this drop_bomb func takes out the cycle import error, but need to figure out how to decrement player bomb num
  pub fn drop_bomb(&mut self) {
    if self.bombs > 0 {
      self.bombs -= 1;
    }
  }

  pub fn position(self) -> (i32, i32) {
    return (self.x, self.y);
  }

  pub fn space(self, x: i32, y: i32) -> bool {
    return (self.x == x) && (self.y == y)
  }

  pub fn id(self) -> i32 {
    return self.id;
  }
 
  pub fn x(self) -> i32 {
    return self.x;
  }

  pub fn y(self) -> i32 {
    return self.y;
  }
}