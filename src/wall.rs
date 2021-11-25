mod bomb;


// Struct for Wallss
  // x: x coord
  // y: y coord
  // destructible: is the wall destrctible
  // alive: remove from Vec<Wall> if not alive 
  struct Wall {
    x: i32,
    y: i32,
    destrctible: bool,
    alive: bool

  }
  
  impl Wall {
  
    // Constructor for walls
    fn new(x: i32, y: i32, destrctible: bool, alive : bool) -> Wall {
      Wall { x: x, y: y, destrctible: destrctible, alive:alive }
    }
  
    // change wall state if is bombed
    fn is_bombed(&self) {
        if self.destrctible {
            self.alive = false;
        }
    }
  }

