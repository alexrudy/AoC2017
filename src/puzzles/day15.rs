#[allow(non_camel_case_types)]
type uint = u64;

static DIVISOR: uint = 2147483647;

#[derive(Debug)]
pub struct Generator {
  factor: uint,
  prevvalue: uint,
  picky: Option<uint>,
}

impl Iterator for Generator {
  type Item = uint;

  fn next(&mut self) -> Option<uint> {
    match self.picky {
      Some(p) => loop {
        if let Some(nv) = self.nextval() {
          if nv % p == 0 {
            return Some(nv);
          }
        }
      },
      None => {
        return self.nextval();
      }
    }
  }
}

impl Generator {
  fn nextval(&mut self) -> Option<uint> {
    let nextvalue = (self.prevvalue * self.factor) % DIVISOR;
    self.prevvalue = nextvalue;
    Some(nextvalue)
  }

  pub fn new(factor: uint, start: uint) -> Generator {
    Generator {
      factor: factor,
      prevvalue: start,
      picky: None,
    }
  }

  pub fn picky(&mut self, value: uint) {
    self.picky = Some(value);
  }
}

static FILTER: uint = 0b1111111111111111;

fn judge(a: uint, b: uint) -> uint {
  if (a & FILTER) == (b & FILTER) {
    1
  } else {
    0
  }
}

pub fn compete(a: &mut Generator, b: &mut Generator, n: usize) -> uint {
  a.zip(b).map(|(av, bv)| judge(av, bv)).take(n).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_part1_first_few() {
    let a = Generator::new(16807, 65);
    let b = Generator::new(48271, 8921);
    assert_eq!(
      a.take(5).collect::<Vec<uint>>(),
      vec![1092455, 1181022009, 245556042, 1744312007, 1352636452]
    );
    assert_eq!(
      b.take(5).collect::<Vec<uint>>(),
      vec![430625591, 1233683848, 1431495498, 137874439, 285222916]
    );
  }

  #[test]
  fn example_part1_first_few_score() {
    let mut a = Generator::new(16807, 65);
    let mut b = Generator::new(48271, 8921);
    assert_eq!(compete(&mut a, &mut b, 5), 1);
  }

  #[test]
  fn example_part1() {
    let mut a = Generator::new(16807, 65);
    let mut b = Generator::new(48271, 8921);
    assert_eq!(compete(&mut a, &mut b, 40e+6 as usize), 588);
  }

  #[test]
  fn example_part2_first_few() {
    let mut a = Generator::new(16807, 65);
    let mut b = Generator::new(48271, 8921);
    a.picky(4);
    b.picky(8);
    assert_eq!(
      a.take(5).collect::<Vec<uint>>(),
      vec![1352636452, 1992081072, 530830436, 1980017072, 740335192]
    );
    assert_eq!(
      b.take(5).collect::<Vec<uint>>(),
      vec![1233683848, 862516352, 1159784568, 1616057672, 412269392]
    );
  }

  #[test]
  fn example_part2_first_few_score() {
    let mut a = Generator::new(16807, 65);
    let mut b = Generator::new(48271, 8921);
    a.picky(4);
    b.picky(8);
    assert_eq!(compete(&mut a, &mut b, 5), 0);
  }

  #[test]
  fn example_part2_first_score() {
    let mut a = Generator::new(16807, 65);
    let mut b = Generator::new(48271, 8921);
    a.picky(4);
    b.picky(8);
    assert_eq!(compete(&mut a, &mut b, 1056), 1);
  }

  #[test]
  fn example_part2() {
    let mut a = Generator::new(16807, 65);
    let mut b = Generator::new(48271, 8921);
    a.picky(4);
    b.picky(8);
    assert_eq!(compete(&mut a, &mut b, 5e+6 as usize), 309);
  }
}
