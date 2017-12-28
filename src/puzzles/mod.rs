//! General Puzzle Solutions
//! for Advent of Code 2017

use std::path;
use std::fs;
use std::io;

/// Get an `io::BufReader` for the puzzle input for the specified day.
pub fn get_puzzle_reader(day: usize) -> io::Result<io::BufReader<fs::File>> {
  let mut filename = path::PathBuf::from(".");
  filename.extend(vec!["puzzles", &format!("{}", day), "input.txt"]);

  let f = fs::File::open(filename)?;
  Ok(io::BufReader::new(f))
}


pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
