use std::io;
#[allow(unused_imports)]
use std::io::prelude::*;

use std::fmt;
use std;
use std::error::Error as StdError;

#[allow(non_camel_case_types)]
type uint = u32;

#[derive(Debug)]
pub enum Error {
  NoItemError,
  NoDivisorsError,
  MultipleDivisorsError((EvenDivide, EvenDivide)),
  IoError(io::Error),
  ParseError(std::num::ParseIntError),
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::IoError(err)
  }
}

impl From<std::num::ParseIntError> for Error {
  fn from(err: std::num::ParseIntError) -> Error {
    Error::ParseError(err)
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::NoItemError => "No Items found",
      Error::NoDivisorsError => "No Items could be evenly divided",
      Error::MultipleDivisorsError(_) => "Two divisors found",
      Error::IoError(_) => "An I/O Error occured",
      Error::ParseError(_) => "Integer parsing error",
    }
  }

  fn cause(&self) -> Option<&std::error::Error> {
    match *self {
      Error::IoError(ref err) => Some(err as &std::error::Error),
      Error::ParseError(ref err) => Some(err as &std::error::Error),
      _ => None,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::NoItemError => f.write_str(self.description()),
      Error::NoDivisorsError => f.write_str(self.description()),
      Error::MultipleDivisorsError((ref a, ref b)) => f.write_str(&format!(
        "{}: {}/{} and {}/{}",
        self.description(),
        a.numerator,
        a.denominator,
        b.numerator,
        b.denominator
      )),
      Error::IoError(ref e) => e.fmt(f),
      Error::ParseError(ref e) => e.fmt(f),
    }
  }
}

type Result<T> = std::result::Result<T, Error>;

/// Checksums a single spreadsheet row.
fn checksum_row(row: &[uint]) -> Result<uint> {
  let biggest = row.iter().max().ok_or(Error::NoItemError)?;
  let smallest = row.iter().min().ok_or(Error::NoItemError)?;

  return Ok(biggest - smallest);
}

#[derive(Debug)]
pub struct EvenDivide {
  numerator: uint,
  denominator: uint,
}

impl EvenDivide {
  fn get(&self) -> uint {
    return self.numerator / self.denominator;
  }

  fn new(numerator: uint, denominator: uint) -> EvenDivide {
    EvenDivide {
      numerator: numerator,
      denominator: denominator,
    }
  }
}

/// Computes the one even divide pair in a row.
fn even_divides_row(row: &[uint]) -> Result<uint> {
  let mut result = None;
  for (i, x) in row.iter().enumerate() {
    for y in row.iter().skip(i + 1) {
      let mut r = None;
      if (y >= x) && (y % x == 0) {
        r = Some(EvenDivide::new(*y, *x));
      } else if (x >= y) && (x % y == 0) {
        r = Some(EvenDivide::new(*x, *y));
      }

      result = match (r, result) {
        (Some(this_result), None) => Some(this_result),
        (Some(this_result), Some(prev_result)) => {
          return Err(Error::MultipleDivisorsError((this_result, prev_result)))
        }
        (None, rv) => rv,
      }
    }
  }
  match result {
    Some(rv) => Ok(rv.get()),
    None => Err(Error::NoDivisorsError),
  }
}

/// Turns a string of row numbers into a vector.
fn parse_row(row: &String) -> Result<Vec<uint>> {
  let mut row_numbers: Vec<uint> = Vec::new();
  for number in row.trim().split(char::is_whitespace) {
    row_numbers.push(number.parse::<uint>()?)
  }
  return Ok(row_numbers);
}

/// Returns the sum of even divisors for a lines of numbers.
pub fn even_divides<T: io::BufRead>(lines: io::Lines<T>) -> Result<uint> {
  let mut checksum = 0;
  for line in lines {
    checksum += even_divides_row(&parse_row(&(line?))?)?
  }
  return Ok(checksum);
}

/// Returns the sum of difference of largest and smallest numbers on lines.
pub fn checksum<T: io::BufRead>(lines: io::Lines<T>) -> Result<uint> {
  let mut checksum = 0;
  for line in lines {
    checksum += checksum_row(&parse_row(&(line?))?)?
  }
  return Ok(checksum);
}

/// Completes both the original checksum and the even divides checksum on a series of rows.
pub fn even_divides_and_checksum<T: io::BufRead>(lines: io::Lines<T>) -> Result<(uint, uint)> {
  let mut checksum = 0;
  let mut divides = 0;
  for line in lines {
    let row = parse_row(&(line?))?;
    checksum += checksum_row(&row)?;
    divides += even_divides_row(&row)?;
  }
  return Ok((checksum, divides));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn even_divides_test() {
    let spreadsheet = "5 9 2 8
9 4 7 3
3 8 6 5";
    assert_eq!(even_divides(spreadsheet.as_bytes().lines()).unwrap(), 9)
  }

  #[test]
  fn full_test() {
    let spreadsheet = "5 1 9 5
7 5 3
2 4 6 8";
    assert_eq!(checksum(spreadsheet.as_bytes().lines()).unwrap(), 18)
  }

  #[test]
  fn full_with_tabs() {
    let spreadsheet = "5	1	9	5
    7	5	3
    2 4	6 8";
    assert_eq!(checksum(spreadsheet.as_bytes().lines()).unwrap(), 18)
  }

  #[test]
  fn parse_1() {
    let row = "2 4 6 8".to_string();
    assert_eq!(parse_row(&row).unwrap(), vec![2, 4, 6, 8]);
  }

  #[test]
  fn checksum_1() {
    let row = vec![5, 1, 9, 5];
    assert_eq!(checksum_row(&row).unwrap(), 8)
  }

  #[test]
  fn checksum_2() {
    let row = vec![7, 5, 3];
    assert_eq!(checksum_row(&row).unwrap(), 4)
  }

  #[test]
  fn checksum_3() {
    let row = vec![2, 4, 6, 8];
    assert_eq!(checksum_row(&row).unwrap(), 6)
  }

}
