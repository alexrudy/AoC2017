extern crate aoc2017;
use aoc2017::puzzles::day19;
use aoc2017::puzzles::day19::Traverseable;
use std::io;
use std::io::BufRead;

fn main() {
  let stdin = io::stdin();

  let map = day19::create_map(stdin.lock().lines());
  let seen_letters: String = map.traverse_letters().collect();
  println!("Part 1: {}", seen_letters);
  println!("Part 2: {} steps", map.traverse().count() + 1);
}
