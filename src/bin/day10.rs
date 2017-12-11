extern crate aoc2017;
use aoc2017::puzzles::day10;

fn main() {
  
  let raw_input = "189,1,111,246,254,2,0,120,215,93,255,50,84,15,94,62";
  let input : Vec<usize> = raw_input.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
  
  // Part 1
  let phash = day10::knot_hash(&input, 256, 1);
  let product = day10::knot_simple_digest(&phash);
  println!("Product: {}", product);
  
  println!("Part 2: {}", day10::knot(&raw_input));
  
}