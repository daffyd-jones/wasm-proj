
mod bomb;

// Struct for Players
  // name: name of player character
  // id: id
  // x: x coord
  // y: y coord
  // bombs: number of bombs
  // power: power level of bombs (how many tiles from center it explodes)
  // alive: is player alive

struct Player {
  name: String,
  id: i32,
  x: i32, 
  y: i32,
  bombs: i32,
  power: i32,
  alive: bool
}

impl Player {
  
  // constructor for new players
  fn new(name: String, id: i32, x: i32, y: i32, bombs: i32, power: i32) -> Player {
    Player {name: name, id: id, x: x, y: y, bombs: bombs, power: power, alive: true}
  }

  // move character to specified coordinates.
  fn move_to(&self, x: i32, y:i32) {
    self.x = x;
    self.y = y;
  }

  // increment position of player
  // TODO: check to make sure within bounds of play
  fn up(&self) {
    self.move_to(self.x, self.y - 1)
  }

  fn down(&self) {
    self.move_to(self.x, self.y + 1)
  }

  fn left(&self) {
    self.move_to(self.x - 1, self.y)
  }

  fn right(&self) {
    self.move_to(self.x + 1, self.y)
  }

  // unalive self.
  fn kill(&self) {
    self.alive = false;
  }

  // get a bomb placed at current location.
  fn drop_bomb(&self) -> Option<bomb::Bomb> {
    if (self.bombs > 0) {
      self.bombs -= 1;
      Some(bomb::Bomb.new(x, y, power))
    }
    None
  }

  
  

}