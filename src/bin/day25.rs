extern crate aoc2017;
use aoc2017::puzzles::day25;
use aoc2017::puzzles;

extern crate failure;
use failure::Error;

use std::io::BufRead;
use std::process;

fn main() {
  
  let mut tm = match get_machine() {
    Ok(t) => {t},
    Err(e) => {
      eprintln!("Error: {:?}", e);
      process::exit(1);
    }
  };
  
  let c = match tm.iter().nth(12173597) {
    Some(count) => {count},
    None => {
      eprintln!("The iterator failed to achieve the right value.");
      process::exit(1);
    }
  };
  println!("Day 25 solution:");
  println!("Part 1: {}", c);
}

fn get_machine() -> Result<day25::TuringMachine, Error> {
  let rules = puzzles::get_puzzle_reader(25)?.lines().collect::<Result<Vec<_>,_>>()?;
  Ok(rules.join("\n").parse()?)
}