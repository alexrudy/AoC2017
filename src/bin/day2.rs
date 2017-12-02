extern crate aoc2017;
use aoc2017::puzzles::day2;
use std::io;
use std::io::prelude::*;

fn main() {
  
  let stdin = io::stdin();
  let checksum = day2::checksum(stdin.lock().lines()).unwrap();
  println!("Checksum: {}", checksum)
}