pub struct WallStruct {
    x: u32,
    y: u32,
    destrctible: bool,
    alive: bool

  }
  
  impl WallStruct {
  
    // Constructor for walls
    pub fn new(x: u32, y: u32, destrctible: bool, alive : bool) -> WallStruct {
      WallStruct { x: x, y: y, destrctible: destrctible, alive:alive }
    }
  
    // change wall state if is bombed
    pub fn is_bombed(&mut self) {
        if self.destrctible {
            self.alive = false;
        }
    }
  }