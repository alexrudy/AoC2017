extern crate aoc2017;
extern crate failure;
use aoc2017::puzzles::day24;
use aoc2017::puzzles;

use std::io::BufRead;
use std::error::Error;


#[test]
fn part_one() {
  
  use aoc2017::puzzles::day24::Bridge;
  
  let connectors = get_connectors().expect("Error loading connectors");
  assert_eq!(day24::bridges(&connectors).map(|b| b.strength()).max(), Some(1906));
}

#[test]
fn part_two() {
  use aoc2017::puzzles::day24::Bridge;
  
  let connectors = get_connectors().expect("Error loading connectors");
  let (_bl, bs) = day24::bridges(&connectors).map(|b| (b.len(), b.strength())).max().unwrap();
  assert_eq!(bs, 1824);
}


fn get_connectors() -> Result<Vec<day24::Connector>, Box<Error>> {
  Ok(puzzles::get_puzzle_reader(24)?.lines().map(|l| Ok(l?.parse::<day24::Connector>()?)).collect::<Result<Vec<day24::Connector>,Box<Error>>>()?)
}