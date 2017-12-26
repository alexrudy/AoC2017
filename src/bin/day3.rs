extern crate aoc2017;
use aoc2017::puzzles::day3;

fn main() {
  let mem = 361527;
  println!("Memory {} takes {} steps.", mem, day3::memory_distance(mem));
  println!(
    "First value above {} is {}",
    mem,
    day3::first_large_value(mem).unwrap()
  );
}
