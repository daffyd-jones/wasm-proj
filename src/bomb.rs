use rand::Rng;

// Struct for Bombs
  // x: x coord
  // y: y coord
  // power: bomb power level
  // timer: count down to explosion
#[derive(Debug, Copy, Clone)]
pub struct BombStruct {
  x: i32,
  y: i32,
  power: i32,
  timer: i32
}

impl BombStruct {

  // Constructor for bombs
  pub fn new(x: i32, y: i32) -> BombStruct {
    // Generate a random number between 1 and 5 to be the power and timer
    let mut rng = rand::thread_rng();
    let rand_pow_time = rng.gen_range(1..6);
    BombStruct { x: x, y: y, power: rand_pow_time, timer: rand_pow_time }
  }

  // Get vector of tuples containing all exploded tiles.
  pub fn explosion_tiles(&self) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    for i in 0..(self.power) {
      v.push((self.x, (self.y + i)));
      v.push((self.x, (self.y - i)));
      v.push(((self.x + i), self.y));
      v.push(((self.x - i), self.y));
    }

    return v;
  }

  // Count down to explosion
  pub fn count_down(&mut self) {
    self.timer -= 1;
  }

  pub fn position(self) -> (i32, i32) {
    return (self.x, self.y);
  }

  pub fn power(self) -> i32 {
    return self.power;
  }

  pub fn timer(self) -> i32 {
    return self.timer;
  }
}

