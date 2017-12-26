use std::collections::HashMap;

#[allow(non_camel_case_types)]
type int = i32;

#[allow(non_camel_case_types)]
type uint = u32;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct MemoryPosition {
  x: int,
  y: int,
}

impl MemoryPosition {
  fn shift(&self, dx: int, dy: int) -> MemoryPosition {
    MemoryPosition {
      x: self.x + dx,
      y: self.y + dy,
    }
  }

  fn mhdistance(&self) -> uint {
    return (self.x.abs() + self.y.abs()) as uint;
  }
}

fn neighbor_sum(
  position: &MemoryPosition,
  grid: &HashMap<MemoryPosition, uint>,
) -> Result<uint, String> {
  let mut nsum: uint = 0;
  for dx in -1..2 {
    for dy in -1..2 {
      nsum = nsum
        .checked_add(*grid.get(&position.shift(dx, dy)).unwrap_or(&0))
        .ok_or("Overflow Error")?;
    }
  }
  Ok(nsum)
}

pub fn first_large_value(position: uint) -> Result<uint, String> {
  let mut dd = (1, 0);
  let mut grid: HashMap<MemoryPosition, uint> = HashMap::new();
  let mut pos = MemoryPosition { x: 0, y: 0 };
  grid.insert(pos, 1);
  for _i in 1.. {
    pos = pos.shift(dd.0, dd.1);
    let val = neighbor_sum(&pos, &grid)?;
    grid.insert(pos, val);
    if !grid.contains_key(&MemoryPosition {
      x: pos.x - dd.1,
      y: pos.y + dd.0,
    }) {
      dd = (-1 * dd.1, dd.0)
    }
    if val > position {
      return Ok(val);
    }
  }
  return Err(format!("No value above {} found", position));
}

pub fn memory_distance(position: uint) -> uint {
  let mut dd = (1, 0);
  let mut grid: HashMap<MemoryPosition, uint> = HashMap::new();
  let mut pos = MemoryPosition { x: 0, y: 0 };
  grid.insert(pos, 1);
  for _i in 1..position {
    pos = pos.shift(dd.0, dd.1);
    grid.insert(pos, 1);
    if !grid.contains_key(&MemoryPosition {
      x: pos.x - dd.1,
      y: pos.y + dd.0,
    }) {
      dd = (-1 * dd.1, dd.0)
    }
  }
  return pos.mhdistance();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple_grid() {
    assert_eq!(memory_distance(1), 0);
    assert_eq!(memory_distance(12), 3);
    assert_eq!(memory_distance(23), 2);
    assert_eq!(memory_distance(1024), 31);
  }

  #[test]
  fn first_val() {}
}
