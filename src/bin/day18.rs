extern crate aoc2017;
use aoc2017::puzzles::day18;
use std::io;
use std::io::Read;

fn main() {
  let mut program = String::new();
  let stdin = io::stdin();
  stdin.lock().read_to_string(&mut program).expect("Read failure!");
  
  match day18::run_program(&program).take(1).next() {
    Some(sound) =>  println!("First sound emitted: {}", sound),
    None => println!("No sound emitted!")
  };
  
  let (a, b) = day18::run_pair(&program);
  println!("a: {} b: {}", a, b)
}
