use std::collections::HashMap;
use std::str::FromStr;
use std::marker::PhantomData;

#[derive(Debug, Fail)]
pub enum VirusError {
  #[fail(display = "Failure to parse cluster: {}", _0)] ParseError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Infection {
  Clean,
  Weakened,
  Infected,
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
    Point {
      x: -self.y,
      y: self.x,
    }
  }

  /// Turn the point to the right.
  fn right(&self) -> Self {
    Point {
      x: self.y,
      y: -self.x,
    }
  }

  /// Reverse direction.
  fn reverse(&self) -> Self {
    Point {
      x: -self.x,
      y: -self.y,
    }
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

  pub fn travel<'a>(&'a mut self) -> VirusIterator<'a, SimpleVirusMode> {
    VirusIterator {
      cluster: self,
      direction: Point::up(),
      position: Point::origin(),
      mode: PhantomData,
    }
  }

  pub fn advnaced_travel<'a>(&'a mut self) -> VirusIterator<'a, AdvancedVirusMode> {
    VirusIterator {
      cluster: self,
      direction: Point::up(),
      position: Point::origin(),
      mode: PhantomData,
    }
  }
}

impl FromStr for Cluster {
  type Err = VirusError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut cluster = Self::new();

    let l = ((s.lines().count() - 1) / 2) as isize;

    for (y, line) in s.lines().enumerate() {
      for (x, _c) in line.chars().enumerate().filter(|&(_x, c)| c == '#') {
        cluster.infected.insert(
          Point {
            x: (x as isize) - l,
            y: l - (y as isize),
          },
          Infection::Infected,
        );
      }
    }

    Ok(cluster)
  }
}

pub trait VirusMode {}
pub struct AdvancedVirusMode;
impl VirusMode for AdvancedVirusMode {}
pub struct SimpleVirusMode;
impl VirusMode for SimpleVirusMode {}

pub struct VirusIterator<'a, T>
where
  T: VirusMode,
{
  cluster: &'a mut Cluster,
  direction: Point,
  position: Point,
  mode: PhantomData<T>,
}

impl<'a> Iterator for VirusIterator<'a, SimpleVirusMode> {
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
      self
        .cluster
        .infected
        .insert(self.position, Infection::Infected);
      Some(true)
    };

    // 3. Move the virus
    self.position.travel(&self.direction);

    // 4. Yeild the desired value.
    result
  }
}

impl<'a> Iterator for VirusIterator<'a, AdvancedVirusMode> {
  type Item = bool;

  fn next(&mut self) -> Option<bool> {
    use std::collections::hash_map::Entry::Occupied;

    // 1. Turn the virus.
    // 2. Toggle the position.
    let result;
    {
      let state = self
        .cluster
        .infected
        .entry(self.position)
        .or_insert(Infection::Clean);
      result = match *state {
        Infection::Clean => {
          self.direction = self.direction.left();
          *state = Infection::Weakened;
          Some(false)
        }
        Infection::Weakened => {
          *state = Infection::Infected;
          Some(true)
        }
        Infection::Infected => {
          self.direction = self.direction.right();
          *state = Infection::Flagged;
          Some(false)
        }
        Infection::Flagged => {
          self.direction = self.direction.reverse();
          *state = Infection::Clean;
          Some(false)
        }
      };
    }

    match self.cluster.infected.entry(self.position) {
      Occupied(entry) => {
        if entry.get() == &Infection::Clean {
          entry.remove();
        }
      }
      _ => {}
    };

    // 3. Move the virus
    self.position.travel(&self.direction);

    // println!("{:?}", self.cluster.infected);
    // 4. Yeild the desired value.
    result
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse_cluster() {
    let cluster: Cluster = "..#\n#..\n...".parse().unwrap();
    assert_eq!(cluster.infected.len(), 2);
    assert!(cluster.infected.contains_key(&Point { x: 1, y: 1 }));
    assert!(cluster.infected.contains_key(&Point { x: -1, y: 0 }));
  }

  #[test]
  fn travel_cluster() {
    let mut cluster: Cluster = "..#\n#..\n...".parse().unwrap();
    let mut traveler = cluster.travel();
    assert!(traveler.next().unwrap());
    assert_eq!(traveler.cluster.infected.len(), 3);
    assert!(!traveler.next().unwrap());
    assert_eq!(traveler.cluster.infected.len(), 2);
    assert!(traveler.take(4).all(|x| x));
  }

  #[test]
  fn travel_cluster_example() {
    let mut cluster: Cluster = "..#\n#..\n...".parse().unwrap();
    assert_eq!(
      cluster.travel().take(70).map(|x| x as usize).sum::<usize>(),
      41
    );
    cluster = "..#\n#..\n...".parse().unwrap();
    assert_eq!(
      cluster
        .travel()
        .take(10000)
        .map(|x| x as usize)
        .sum::<usize>(),
      5587
    );
  }

  #[test]
  fn advanced_cluster_example() {
    let mut cluster: Cluster = "..#\n#..\n...".parse().unwrap();
    assert_eq!(
      cluster
        .advnaced_travel()
        .take(100)
        .map(|x| x as usize)
        .sum::<usize>(),
      26
    );
  }
}
