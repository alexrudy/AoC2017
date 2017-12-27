extern crate aoc2017;
use aoc2017::puzzles::day21;

extern crate failure;

use std::io;
use std::io::BufRead;
use std::process;

fn load_patterns() -> Result<day21::PatternSet, failure::Error> {
  let stdin = io::stdin();
  let mut patterns = day21::PatternSet::new();
  for line in stdin.lock().lines() {
    patterns.insert(&line?)?;
  }
  Ok(patterns)
}

fn main() {
  let patterns = match load_patterns() {
    Ok(p) => p,
    Err(e) => {
      eprintln!("Error: {:?}", e);
      process::exit(1);
    }
  };

  match day21::after_n(&patterns, 5).map(|p| day21::on(&p)) {
    Ok(ans) => println!("Part 1: {} cells are on after 5 iterations", ans),
    Err(e) => {
      eprintln!("Error: {:?}", e);
      process::exit(1);
    }
  };

  match day21::after_n(&patterns, 18).map(|p| day21::on(&p)) {
    Ok(ans) => println!("Part 2: {} cells are on after 18 iterations", ans),
    Err(e) => {
      eprintln!("Error: {:?}", e);
      process::exit(1);
    }
  };
}
