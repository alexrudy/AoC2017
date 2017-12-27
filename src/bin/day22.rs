extern crate aoc2017;
use aoc2017::puzzles::day22;

use std::io;
use std::io::Read;

fn main() {
  let stdin = io::stdin();
  let mut cluster_init = String::new();
  stdin
    .lock()
    .read_to_string(&mut cluster_init)
    .expect("Reading a string map.");

  {
    let mut cluster: day22::Cluster = cluster_init.parse().expect("Trouble parsing");
    println!(
      "Part 1: {} infections",
      cluster
        .travel()
        .take(10000)
        .map(|x| x as usize)
        .sum::<usize>()
    );
  }

  {
    let mut cluster: day22::Cluster = cluster_init.parse().expect("Trouble parsing");
    println!(
      "Part 2: {} infections",
      cluster
        .advnaced_travel()
        .take(10000000)
        .map(|x| x as usize)
        .sum::<usize>()
    );
  }
}
