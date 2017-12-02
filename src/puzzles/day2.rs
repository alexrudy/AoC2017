use std::io;

#[allow(unused_imports)]
use std::io::prelude::*;

/// Checksums a single spreadsheet row.
fn checksum_row(row: &[u32]) -> Result<u32, String> {
  let biggest = row.iter().max().ok_or("No maximum!".to_string())?;
  let smallest = row.iter().min().ok_or("No minimum!".to_string())?;
  
  return Ok(biggest - smallest)
}

/// Turns a string of row numbers into a vector.
fn parse_row(row: String) -> Result<Vec<u32>, String> {
  let mut row_numbers : Vec<u32> = Vec::new();
  for number in row.trim().split(char::is_whitespace) {
    match number.parse::<u32>() {
      Ok(n) => row_numbers.push(n),
      Err(_) => return Err(format!("Can't parse {}", number)),
    }
  }
  return Ok(row_numbers)
}

fn checksum_string_row(row: String) -> Result<u32, String> {
  let row_numbers = parse_row(row)?;
  return checksum_row(&row_numbers);
}

pub fn checksum<T: io::BufRead>(lines: io::Lines<T>) -> Result<u32, String> {
  let mut checksum = 0;
  for line in lines {
    match line {
      Ok(row) => checksum += checksum_string_row(row)?,
      Err(err) => return Err(err.to_string())
    }
  }
  return Ok(checksum)
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn full_test() {
    let spreadsheet = "5 1 9 5
7 5 3
2 4 6 8";
    assert_eq!(checksum(spreadsheet.as_bytes().lines()), Ok(18))
    
  }
  
  #[test]
  fn parse_1() {
    let row = "2 4 6 8".to_string();
    assert_eq!(parse_row(row).expect("row"), vec![2,4,6,8]);
  }
  
  #[test]
  fn checksum_1() {
    let row = vec![5,1,9,5];
    assert_eq!(checksum_row(&row).expect("answer"), 8)
  }
  
  #[test]
  fn checksum_2() {
    let row = vec![7,5,3];
    assert_eq!(checksum_row(&row).expect("answer"), 4)
  }
  
  #[test]
  fn checksum_3() {
    let row = vec![2,4,6,8];
    assert_eq!(checksum_row(&row).expect("answer"), 6)
  }
  
}