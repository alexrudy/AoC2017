extern crate aoc2017;

use std::fs::File;
use std::io::Read;
use aoc2017::puzzles::day18;

#[test]
fn part_one() {
  let mut program = String::new();
  let mut f = File::open("puzzles/18/input.txt").expect("file not found");
  f.read_to_string(&mut program).expect("Read failure!");
  assert_eq!(day18::run_program(&program).take(1).next(), Some(2951));
}

#[test]
fn part_two() {
  let mut program = String::new();
  let mut f = File::open("puzzles/18/input.txt").expect("file not found");
  f.read_to_string(&mut program).expect("Read failure!");
  assert_eq!(day18::run_pair(&program).1, 7366);
}