use std::io;
#[allow(unused_imports)]
use std::io::prelude::*;

use std::fmt;  
use std;

#[derive(Debug)]
pub enum Error {  
    NoItemError,
    NoDivisorsError,
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for Error {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoItemError => f.write_str("No Items Found"),
            Error::NoDivisorsError => f.write_str("No Items could be evenly divided"),
            Error::IoError(ref e) => e.fmt(f),
            Error::ParseError(ref e) => e.fmt(f),
        }
    }
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
            Error::NoItemError => "No Items Found",
            Error::NoDivisorsError => "No Items could be evenly divided.",
            Error::IoError(_) => "An I/O Error Occured",
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

type Result<T> = std::result::Result<T, Error>;

/// Checksums a single spreadsheet row.
fn checksum_row(row: &[u32]) -> Result<u32> {
  let biggest = row.iter().max().ok_or(Error::NoItemError)?;
  let smallest = row.iter().min().ok_or(Error::NoItemError)?;
  
  return Ok(biggest - smallest)
}

fn even_divides_row(row: &[u32]) -> Result<u32> {
  for (i, x) in row.iter().enumerate() {
    for y in row.iter().skip(i + 1) {
      if (y >= x) && (y % x == 0) {
        return Ok(y / x);
      } else if (x >= y) && (x % y == 0) {
        return Ok(x / y);
      }
    }
  }
  return Err(Error::NoDivisorsError)
}

/// Turns a string of row numbers into a vector.
fn parse_row(row: &String) -> Result<Vec<u32>> {
  let mut row_numbers : Vec<u32> = Vec::new();
  for number in row.trim().split(char::is_whitespace) {
    row_numbers.push(number.parse::<u32>()?)
  }
  return Ok(row_numbers)
}

fn checksum_string_row(row: &String) -> Result<u32> {
  let row_numbers = parse_row(row)?;
  return checksum_row(&row_numbers);
}

fn even_divides_string_row(row: &String) -> Result<u32> {
  let row_numbers = parse_row(row)?;
  return even_divides_row(&row_numbers);
}

pub fn even_divides<T: io::BufRead>(lines: io::Lines<T>) -> Result<u32> {
  let mut checksum = 0;
  for line in lines {
    checksum += even_divides_string_row(&(line?))?
  }
  return Ok(checksum)
}

pub fn checksum<T: io::BufRead>(lines: io::Lines<T>) -> Result<u32> {
  let mut checksum = 0;
  for line in lines {
    checksum += checksum_string_row(&(line?))?
  }
  return Ok(checksum)
}

pub fn even_divides_and_checksum<T: io::BufRead>(lines: io::Lines<T>) -> Result<(u32, u32)> {
  let mut checksum = 0;
  let mut divides = 0;
  for line in lines {
    let row = line?;
    checksum += checksum_string_row(&row)?;
    divides += even_divides_string_row(&row)?;
  }
  return Ok((checksum, divides))
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
    assert_eq!(parse_row(&row).unwrap(), vec![2,4,6,8]);
  }
  
  #[test]
  fn checksum_1() {
    let row = vec![5,1,9,5];
    assert_eq!(checksum_row(&row).unwrap(), 8)
  }
  
  #[test]
  fn checksum_2() {
    let row = vec![7,5,3];
    assert_eq!(checksum_row(&row).unwrap(), 4)
  }
  
  #[test]
  fn checksum_3() {
    let row = vec![2,4,6,8];
    assert_eq!(checksum_row(&row).unwrap(), 6)
  }
  
}