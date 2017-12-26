extern crate aoc2017;
use aoc2017::puzzles::day9;
use std::io;
use std::io::prelude::*;

fn main() {
  let mut stdin = io::stdin();
  let mut text = String::new();
  stdin.read_to_string(&mut text).unwrap();

  let (score, garbage) = day9::procress_stream(&text);
  println!("Score: {}", score);
  println!("Garbage: {}", garbage);
}
