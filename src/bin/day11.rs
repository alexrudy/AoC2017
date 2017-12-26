extern crate aoc2017;
use aoc2017::puzzles::day11;
use std::io;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  let mut moves = String::new();

  stdin.lock().read_to_string(&mut moves).unwrap();

  println!(
    "Moves to get to the child process: {}",
    day11::distance(&moves).unwrap()
  );
  println!(
    "Maximum distance for child: {}",
    day11::maxdistance(&moves).unwrap()
  );
}
