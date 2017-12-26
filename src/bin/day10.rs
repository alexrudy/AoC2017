extern crate aoc2017;
use aoc2017::puzzles::day10;

fn main() {
  let raw_input = "189,1,111,246,254,2,0,120,215,93,255,50,84,15,94,62";
  println!("Product: {}", day10::part1(raw_input));

  println!("Part 2: {}", day10::part2(raw_input).hexdigest());
}
