
// Struct for Bombs
  // x: x coord
  // y: y coord
  // power: bomb power level
struct Bomb {
  x: i32,
  y: i32,
  power: i32,
}

impl Bomb {

  // Constructor for bombs
  fn new(x: i32, y: i32, pow: i32) -> Bomb {
    Bomb { x: x, y: y, power: pow }
  }

  // Get vector of tuples containing all exploded tiles.
  fn explosion_tiles(&self) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    for i in 0..(self.power) {
      v.push((self.x, (self.y + i)));
      v.push((self.x, (self.y - i)));
      v.push(((self.x + i), self.y));
      v.push(((self.x - i), self.y));
    }

    return v;
  }
}