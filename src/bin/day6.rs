extern crate aoc2017;
use aoc2017::puzzles::day6;
use std::io;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  let mut line = String::new();
  stdin.lock().read_line(&mut line).unwrap();
  let mut memory : Vec<usize> = line.trim().split(char::is_whitespace).map(|x| x.parse::<usize>().unwrap()).collect();
  
  println!("Memory {:?}", memory);
  let (nsteps, ncycle) = day6::reallocate_many(&mut memory);
  
  println!("In {} steps, found a repeat state.", nsteps);
  println!("The cycle is {} steps long.", ncycle);
}