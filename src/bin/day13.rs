extern crate aoc2017;
use aoc2017::puzzles::day13;
use std::io;

fn main() {
  let stdin = io::stdin();

  let path = day13::parse(stdin.lock());

  println!("The severity (t=0) is {}", day13::severity(0, &path));
  println!("Can sneak through at t={}", day13::sneak(&path));
}
