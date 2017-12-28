//! An implementation of "Knot Hashing" for use with advent of code 2017.
//!
//! The primary struct in this module, `Knot`, should be used to handle knot
//! hashing and produce digests.

/// A knot hash
pub struct Knot {
  hash: Vec<u8>,
  offset: usize,
  skip: usize,
}

impl Knot {
  /// Compute the knot hash once.
  fn once(&mut self, length: usize) {
    let split = length % self.hash.len();
    {
      let (segment, _) = self.hash.split_at_mut(split);
      segment.reverse();
    }
    let offset = (length + self.skip) % self.hash.len();
    self.hash.rotate(offset);

    self.offset += length + self.skip
  }

  /// Extend the knot hash inputs with the
  /// standard input extension, `[17, 31, 73, 47, 23]`
  fn extend(inputs: &mut Vec<u8>) {
    inputs.extend(vec![17, 31, 73, 47, 23]);
  }

  /// Return the hash for this Knot,
  /// as an array of `u8` items.
  pub fn hash(&self) -> &[u8] {
    return &self.hash;
  }

  /// The binary digest of this Knot
  /// in xor-d chuncks of 16 as binary digits.
  pub fn bindigest(&self) -> String {
    self
      .hash
      .chunks(16)
      .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
      .map(|x| format!("{:08b}", x))
      .collect()
  }

  /// The hexadecimal digest of this Knot
  /// in xor-d chuncks of 16 as hex.
  pub fn hexdigest(&self) -> String {
    self
      .hash
      .chunks(16)
      .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
      .map(|x| format!("{:02x}", x))
      .collect()
  }

  /// The simple-digest is the prodcut of the first
  /// two numbers in the knot hash.
  pub fn simpledigest(&self) -> u32 {
    let mut diter = self.hash.iter().take(2);
    let a = *diter.next().unwrap() as u32;
    let b = *diter.next().unwrap() as u32;
    a * b
  }

  /// Compute a knot hash over a set number of rounds
  /// for a given input. Applies the knot hash to the
  /// current hash vector as set up in the constructor
  /// for this Knot.
  pub fn compute(&mut self, inputs: &[usize], rounds: usize) {
    let length = self.hash.len();

    for _i in 0..rounds {
      for input in inputs {
        self.once(*input);
        self.skip += 1;
      }
    }

    let offrotate = (length as isize - (self.offset as isize % length as isize)).abs() as usize;
    self.hash.rotate(offrotate);
  }

  /// Create a new Knot hash with a given size
  /// of the hash array for knotting.
  pub fn new(length: usize) -> Knot {
    Knot {
      hash: (0..length).map(|x| x as u8).collect(),
      offset: 0,
      skip: 0,
    }
  }

  /// Compute the standard knot hash over
  /// a string input. The string input is converted
  /// to bytes, then extended by the standard extension
  /// vector, then used as input to a Knot hash of length
  /// 256, applied 64 times.
  pub fn standard(inputs: &str) -> Knot {
    // Prepare input
    let mut binputs: Vec<u8> = inputs.bytes().collect();
    Knot::extend(&mut binputs);
    let uinputs: Vec<usize> = binputs.iter().map(|&x| x as usize).collect();

    // Compute hash
    let mut knot = Knot::new(256);
    knot.compute(&uinputs, 64);

    knot
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn try_hashes() {
    assert_eq!(
      &Knot::standard("AoC 2017").hexdigest(),
      "33efeb34ea91902bb2f59c9920caa6cd"
    );
    assert_eq!(
      &Knot::standard("1,2,3").hexdigest(),
      "3efbe78a8d82f29979031a4aa0b16a9d"
    );
    assert_eq!(
      &Knot::standard("1,2,4").hexdigest(),
      "63960835bcdc130f0b66d7ff4f6a5a8e"
    );
    assert_eq!(
      &Knot::standard("").hexdigest(),
      "a2582a3a0e66e6e86e3812dcb672a272"
    );
  }

  #[test]
  fn do_knot_hash() {
    let inputs = vec![3, 4, 1, 5];

    let mut knot = Knot::new(5);
    knot.compute(&inputs, 1);
    assert_eq!(knot.simpledigest(), 12);
  }

  #[test]
  fn do_one_knot() {
    let mut knot = Knot::new(5);
    knot.compute(&vec![3], 1);
    let expected: Vec<u8> = vec![2, 1, 0, 3, 4];
    assert_eq!(knot.hash(), expected.as_slice());
  }
}
