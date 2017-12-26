extern crate aoc2017;
use aoc2017::puzzles::day21;

use std::io;
use std::io::BufRead;

fn main() {
  
  let stdin = io::stdin();
  let mut patterns = day21::PatternSet::new();
  for line in stdin.lock().lines() {
    patterns.insert(&line.expect("Error reading line")).expect("Error handling rule.");
  }
  
  
}
