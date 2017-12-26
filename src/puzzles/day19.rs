use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point(isize, isize);

type NetworkMap = HashMap<Point, char>;

pub fn create_map<T: io::BufRead>(lines: io::Lines<T>) -> NetworkMap {
  let mut map = NetworkMap::new();

  for (i, line) in lines.enumerate() {
    for (j, c) in line.expect("Readable line!").chars().enumerate() {
      match c {
        ' ' => {} // Skip spaces, all others matter.
        _ => {
          map.insert(Point(j as isize, i as isize), c);
        }
      }
    }
  }
  map
}

pub trait Traverseable {
  fn start(&self) -> Option<Point>;
  fn traverse<'a>(&'a self) -> NetworkPathIterator<'a>;
  fn traverse_letters<'a>(&'a self) -> NetworkPathIterator<'a>;
}

impl Traverseable for NetworkMap {
  fn start(&self) -> Option<Point> {
    for (point, _c) in self {
      if point.1 == 0 {
        return Some(*point);
      }
    }
    None
  }

  fn traverse<'a>(&'a self) -> NetworkPathIterator<'a> {
    NetworkPathIterator {
      position: self.start().expect("Can't locate the start of the graph!"),
      direction: Direction::Up,
      map: &self,
      letters: false,
    }
  }

  fn traverse_letters<'a>(&'a self) -> NetworkPathIterator<'a> {
    NetworkPathIterator {
      position: self.start().expect("Can't locate the start of the graph!"),
      direction: Direction::Up,
      map: &self,
      letters: true,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn turns(&self) -> (Direction, Direction) {
    match *self {
      Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
      Direction::Left | Direction::Right => (Direction::Up, Direction::Down),
    }
  }

  fn progress(&self, point: &Point) -> Point {
    match *self {
      Direction::Up => Point(point.0, point.1 + 1),
      Direction::Down => Point(point.0, point.1 - 1),
      Direction::Left => Point(point.0 - 1, point.1),
      Direction::Right => Point(point.0 + 1, point.1),
    }
  }

  fn turn(&self, point: &Point, map: &NetworkMap) -> Result<Direction, String> {
    match map.contains_key(&self.progress(point)) {
      true => Ok(*self),
      false => {
        let (l, r) = self.turns();
        if map.contains_key(&l.progress(point)) {
          Ok(l)
        } else if map.contains_key(&r.progress(point)) {
          Ok(r)
        } else {
          Err("No valid directions...".to_string())
        }
      }
    }
  }
}

// Iterator over the characters in a path.
pub struct NetworkPathIterator<'a> {
  position: Point,
  direction: Direction,
  map: &'a NetworkMap,
  letters: bool,
}

impl<'a> NetworkPathIterator<'a> {
  /// Checks a position for validity, returning the character in
  /// that position if it is valid.
  fn checkposition(&mut self, position: &Point) -> Option<char> {
    match self.map.get(position) {
      Some(item) => {
        self.position = *position;
        Some(*item)
      }
      None => None,
    }
  }

  fn nextstep(&mut self) -> Result<char, String> {
    let position = self.position;
    let nextposition = self.direction.progress(&position);
    let nextitem = self.checkposition(&nextposition).map_or_else(
      || {
        self.direction = match self.direction.turn(&position, self.map) {
          Ok(direction) => direction,
          Err(e) => {
            return Err(e);
          }
        };
        let nextposition = self.direction.progress(&position);
        self
          .checkposition(&nextposition)
          .ok_or("No possible paths".to_string())
      },
      |x| Ok(x),
    );
    nextitem
  }
}

impl<'a> Iterator for NetworkPathIterator<'a> {
  type Item = char;

  fn next(&mut self) -> Option<char> {
    loop {
      match self.nextstep() {
        Ok(c) => {
          if !self.letters || c.is_alphabetic() {
            return Some(c);
          }
        }
        Err(_) => return None,
      }
    }
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use std::io::BufRead;

  #[test]
  fn parse_test() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mapdata = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
    let map = create_map(mapdata.as_bytes().lines());
    assert_eq!(map.len(), 35);
  }

  #[test]
  fn traverse_letters_test() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mapdata = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
    let map = create_map(mapdata.as_bytes().lines());
    let seen_letters: String = map.traverse_letters().collect();
    assert_eq!(&seen_letters, "ABCDEF")
  }

  #[test]
  fn traverse_path_length_test() {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let mapdata = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
    let map = create_map(mapdata.as_bytes().lines());
    assert_eq!(map.traverse().count() + 1, 38);
  }
}
