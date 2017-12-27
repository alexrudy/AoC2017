extern crate aoc2017;
use aoc2017::puzzles::day22;

use std::fs::File;
use std::io::Read;

fn load_infections() -> Result<day22::Cluster, day22::VirusError> {
  let mut cluster = String::new();
  let mut f = File::open("puzzles/22/input.txt").expect("file not found");
  f.read_to_string(&mut cluster).unwrap();
  cluster.parse()
}

#[test]
fn part_one() {
  let mut cluster = load_infections().unwrap();
  assert_eq!(
    cluster
      .travel()
      .take(10000)
      .map(|x| x as usize)
      .sum::<usize>(),
    5240
  );
}

#[test]
fn part_two_example() {
  let mut cluster: day22::Cluster = "..#\n#..\n...".parse().unwrap();
  assert_eq!(
    cluster
      .advnaced_travel()
      .take(10000000)
      .map(|x| x as usize)
      .sum::<usize>(),
    2511944
  );
}

#[test]
fn part_two() {
  let mut cluster = load_infections().unwrap();
  assert_eq!(
    cluster
      .advnaced_travel()
      .take(10000000)
      .map(|x| x as usize)
      .sum::<usize>(),
    2512144
  );
}
