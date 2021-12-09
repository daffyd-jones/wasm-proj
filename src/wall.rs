use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct WallStruct {
    x: i32,
    y: i32,
    destrctible: bool,
    alive: bool

  }
  
  impl WallStruct {
  
    // Constructor for walls
    pub fn new(x: i32, y: i32, destrctible: bool, alive : bool) -> WallStruct {
      WallStruct { x: x, y: y, destrctible: destrctible, alive:alive }
    }
  
    // change wall state if is bombed
    pub fn is_bombed(&mut self) {
        if self.destrctible {
            self.alive = false;
        }
    }
    pub fn x(self) -> i32 {
        return self.x;
      }
    
      pub fn y(self) -> i32 {
        return self.y;
      }

  }