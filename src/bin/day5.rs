extern crate aoc2017;
use aoc2017::puzzles::day5;
use std::io;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  let mut instructions: Vec<isize> = stdin.lock().lines().map(|l| l.unwrap().parse::<isize>().unwrap()).collect();
  
  let mut instructions_fancy = instructions.clone();
  
  let nsteps = day5::run_instructions(&mut instructions, 0, day5::jump_instruction).unwrap();
  println!("It took {} steps to exit the maze.", nsteps);

  let nsteps_fancy = day5::run_instructions(&mut instructions_fancy, 0, day5::jump_fancy).unwrap();
  println!("It took {} steps to exit the maze with fancy instructions.", nsteps_fancy);
  
  
}