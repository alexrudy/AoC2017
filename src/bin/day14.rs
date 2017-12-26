extern crate aoc2017;
use aoc2017::puzzles::day14;

fn main() {
  let disk = day14::Disk::new("amgozmfv");
  println!("{} cells used.", disk.used());
  println!("{} regions found.", disk.regions());
}
