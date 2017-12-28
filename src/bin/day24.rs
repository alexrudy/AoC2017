extern crate aoc2017;
extern crate failure;
use aoc2017::puzzles::day24;
use aoc2017::puzzles;

use std::io::BufRead;
use std::error::Error;
use std::process;

fn main() {
  
  use aoc2017::puzzles::day24::Bridge;
  
  let connectors = match get_connectors() {
    Ok(c) => {c},
    Err(e) => {
      eprintln!("Error loading connectors: {:?}", e);
      process::exit(1);
    }
  };
  
  match day24::bridges(&connectors).map(|b| b.strength()).max() {
    Some(bs) => { 
      println!("Part 1: Strongest bridge has strength {}", bs);},
    None => {
      eprintln!("No bridges can be produced by connectors {:?}!", connectors);
      process::exit(1)
    }
  };
  
  match day24::bridges(&connectors).map(|b| (b.len(), b.strength())).max() {
    Some((bl, bs)) => { 
      println!("Part 2: Longest bridge has length {} and strength {}", bl, bs);},
    None => {
      eprintln!("No bridges can be produced by connectors {:?}!", connectors);
      process::exit(1)
    }
  };
  
  
  
  
}


fn get_connectors() -> Result<Vec<day24::Connector>, Box<Error>> {
  Ok(puzzles::get_puzzle_reader(24)?.lines().map(|l| Ok(l?.parse::<day24::Connector>()?)).collect::<Result<Vec<day24::Connector>,Box<Error>>>()?)
}