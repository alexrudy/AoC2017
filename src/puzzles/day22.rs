use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Fail)]
pub enum VirusError {
  
  #[fail(display = "Failure to parse cluster: {}", _0)]
  ParseError(String),

}

#[derive(Debug)]
enum Infection {
  Infected,
  Weakened,
  Flagged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
  x: isize,
  y: isize,
}

impl Point {
  
  fn origin() -> Self {
    Point { x: 0, y: 0 }
  }
  
  fn up() -> Self {
    Point { x: 0, y: 1 }
  }
  
  /// Turn the point to the left.
  fn left(&self) -> Self {
    Point { x: -self.y, y: self.x }
  }
  
  /// Turn the point to the right.
  fn right(&self) -> Self {
    Point { x: self.y, y: -self.x }
  }
  
  /// Move the point by the given vector.
  fn travel(&mut self, direction: &Self) {
    self.x += direction.x;
    self.y += direction.y;
  }
}

#[derive(Debug)]
pub struct Cluster {
  infected: HashMap<Point, Infection>,
}

impl Cluster {
  pub fn new() -> Self {
    Self {
      infected: HashMap::new(),
    }
  }
  
  pub fn travel<'a>(&'a mut self) -> VirusIterator<'a> {
    VirusIterator {
      cluster: self,
      direction: Point::up(),
      position: Point::origin(),
    }
  }
}

impl FromStr for Cluster {
  type Err = VirusError;
  
  fn from_str(s: &str) -> Result<Self,Self::Err> {
    let mut cluster = Self::new();
    
    let l = ((s.lines().count() - 1) / 2) as isize;
    
    for (y, line) in s.lines().enumerate() {
      for (x, _c) in line.chars().enumerate().filter(|&(_x, c)| c == '#') {
        cluster.infected.insert(Point {x: (x as isize) - l, y: l - (y as isize)}, Infection::Infected);
      }
    }
    
    Ok(cluster)
  }
}

pub struct VirusIterator<'a> {
  cluster: &'a mut Cluster,
  direction: Point,
  position: Point,
}

impl<'a> Iterator for VirusIterator<'a> {
  type Item = bool;
  
  fn next(&mut self) -> Option<bool> {
    
    
    // 1. Turn the virus.
    // 2. Toggle the position.
    let result = if self.cluster.infected.contains_key(&self.position) {
      self.direction = self.direction.right();
      self.cluster.infected.remove(&self.position);
      Some(false)
    } else {
      self.direction = self.direction.left();
      self.cluster.infected.insert(self.position, Infection::Infected);
      Some(true)
    };
    
    // 3. Move the virus
    self.position.travel(&self.direction);
    
    // 4. Yeild the desired value.
    result
  }
}

#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn parse_cluster() {
    let cluster : Cluster = "..#\n#..\n...".parse().unwrap();
    assert_eq!(cluster.infected.len(), 2);
    assert!(cluster.infected.contains_key(&Point { x: 1, y: 1}));
    assert!(cluster.infected.contains_key(&Point { x: -1, y: 0}));
  }
  
  #[test]
  fn travel_cluster() {
    let mut cluster : Cluster = "..#\n#..\n...".parse().unwrap();
    let mut traveler = cluster.travel();
    assert!(traveler.next().unwrap());
    assert_eq!(traveler.cluster.infected.len(), 3);
    assert!(!traveler.next().unwrap());
    assert_eq!(traveler.cluster.infected.len(), 2);
    assert!(traveler.take(4).all(|x| x));
  }
  
  #[test]
  fn travel_cluster_example() {
    let mut cluster : Cluster = "..#\n#..\n...".parse().unwrap();
    assert_eq!(cluster.travel().take(70).map(|x| x as usize).sum::<usize>(), 41);
    cluster = "..#\n#..\n...".parse().unwrap();
    assert_eq!(cluster.travel().take(10000).map(|x| x as usize).sum::<usize>(), 5587);
  }
}