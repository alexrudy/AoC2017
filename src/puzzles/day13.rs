use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Scanner {
  range: usize,
}

impl Scanner {
  /// New scanner, all scanners start at 0.
  pub fn new(range: usize) -> Scanner {
    Scanner { range: range }
  }

  /// Scanner position at time t with range r
  /// t % (2*r) = [0, 1, 2, 3] [4, 5]
  pub fn scan(&self, time: usize) -> usize {
    let mut pos = time % ((self.range * 2) - 2);
    if pos >= self.range {
      pos = 2 * self.range - 1 - pos;
    }
    pos
  }

  pub fn range(&self) -> usize {
    self.range
  }
}

pub fn severity(offset: usize, path: &HashMap<usize, Scanner>) -> usize {
  let mut severity = 0;
  for (depth, scanner) in path {
    if scanner.scan(*depth + offset) == 0 {
      severity += *depth * scanner.range();
    }
  }
  severity
}

pub fn parse<T>(lines: T) -> HashMap<usize, Scanner>
where
  T: io::BufRead,
{
  let mut path = HashMap::new();

  for line in lines.lines() {
    let l = line.unwrap();
    let mut parts = l.split(':');
    let depth = parts.next().unwrap().trim().parse::<usize>().unwrap();
    let range = parts.next().unwrap().trim().parse::<usize>().unwrap();
    path.insert(depth, Scanner::new(range));
  }
  path
}

fn can_sneak(offset: usize, path: &HashMap<usize, Scanner>) -> bool {
  !path
    .iter()
    .any(|(depth, scanner)| scanner.scan(*depth + offset) == 0)
}

pub fn sneak(path: &HashMap<usize, Scanner>) -> usize {
  (0..).find(|x| can_sneak(*x, path)).unwrap()
}

#[cfg(test)]
mod tests {

  use super::*;

  fn example_path() -> HashMap<usize, Scanner> {
    let mut path = HashMap::new();
    path.insert(0, Scanner::new(3));
    path.insert(1, Scanner::new(2));
    path.insert(4, Scanner::new(4));
    path.insert(6, Scanner::new(4));
    path
  }

  #[test]
  fn test_parse() {
    let input = "0: 3
1: 2
4: 4
6: 4"
      .as_bytes();

    let path = example_path();
    assert_eq!(path, parse(input));
  }

  #[test]
  fn test_severity() {
    let path = example_path();
    assert_eq!(severity(0, &path), 24);
  }

  #[test]
  fn test_sneak() {
    let path = example_path();
    assert_eq!(sneak(&path), 10);
  }

}
