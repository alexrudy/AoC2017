extern crate aoc2017;
use aoc2017::puzzles::day21;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_patterns() -> Result<day21::PatternSet, day21::PatchError> {
  let f = File::open("puzzles/21/input.txt").expect("file not found");
  let reader = BufReader::new(f);
  let mut patterns = day21::PatternSet::new();
  for line in reader.lines() {
    patterns.insert(&line.map_err(|e| day21::PatchError::Io(e))?)?;
  }
  Ok(patterns)
}

#[test]
fn part_one() {
  assert_eq!(
    load_patterns()
      .and_then(|patterns| day21::after_n(&patterns, 5).map(|p| day21::on(&p)))
      .unwrap(),
    190
  );
}

#[test]
fn part_two() {
  assert_eq!(
    load_patterns()
      .and_then(|patterns| day21::after_n(&patterns, 18).map(|p| day21::on(&p)))
      .unwrap(),
    2335049
  );
}
