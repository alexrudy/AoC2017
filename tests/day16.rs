
extern crate aoc2017;
use aoc2017::puzzles::day16;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[test]
fn part_two_check_cycle() {
  let f = File::open("puzzles/16/input.txt").expect("file not found");
  let reader = BufReader::new(f);
  let steps: Vec<String> = reader
    .split(',' as u8)
    .map(|s| String::from_utf8(s.unwrap()).unwrap())
    .collect();
  let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
  let (offset, cycle) = day16::dance_cycle(&mut programs, &steps);
  programs = "abcdefghijklmnop".chars().collect();
  for _i in 0..offset {
    day16::dance(&mut programs, &steps);
  }
  let or = day16::dance_string(&programs);
  for _i in 0..cycle {
    day16::dance(&mut programs, &steps);
  }
  assert_eq!(or, day16::dance_string(&programs))
}