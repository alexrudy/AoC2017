
#[derive(Debug, Clone, Copy)]
pub struct HexPoint {
  x: i32,
  y: i32,
  z: i32
}


impl HexPoint {
  
  pub fn origin() -> HexPoint {
    HexPoint { x: 0, y: 0, z: 0}
  }
  
  /// Move to a new hex position, given a string representing a movement.
  pub fn hexmove(&self, direction: &str) -> Result<HexPoint, String> {
    match direction {
      "ne" => Ok(HexPoint { x: self.x, y: self.y + 1, z: self.z - 1}),
      "n"  => Ok(HexPoint { x: self.x - 1, y: self.y + 1, z: self.z}),
      "nw" => Ok(HexPoint { x: self.x - 1, y: self.y, z: self.z + 1}),
      "se" => Ok(HexPoint { x: self.x + 1, y: self.y, z: self.z - 1}),
      "s"  => Ok(HexPoint { x: self.x + 1, y: self.y - 1, z: self.z}),
      "sw" => Ok(HexPoint { x: self.x, y: self.y - 1, z: self.z + 1}),
      _    => Err(format!("Can't understand {}", direction))
    }
  }
  
  pub fn distance(&self, other: HexPoint) -> i32 {
    ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
  }
  
}

