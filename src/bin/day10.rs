extern crate aoc2017;
use aoc2017::puzzles::day10;

fn main() {
  let input = vec![189,1,111,246,254,2,0,120,215,93,255,50,84,15,94,62];
  
  let product = day10::knot_hash(&input, 256);
  println!("Product: {}", product);
  
}