extern crate aoc2017;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use aoc2017::puzzles::day20;

#[test]
fn part_one() {
  let f = File::open("puzzles/20/input.txt").expect("file not found");
  let reader = BufReader::new(f);
  let particles : Vec<day20::Particle> = reader.lines()
  .map(|line| day20::Particle::parse(&line.unwrap()).unwrap())
  .collect();
  assert_eq!(day20::find_closest(particles.clone()), 376);
}