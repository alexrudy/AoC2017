extern crate aoc2017;
use aoc2017::puzzles::day8;
use std::io;
use std::cmp;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  let mut registers = day8::Registers::new();
  let mut maxval = None;
  for line in stdin.lock().lines() {
    let instruction = day8::Instruction::parse(&line.unwrap()).unwrap();
    {
      instruction.execute(&mut registers);
    }
    let thismax = registers.values().max().unwrap().clone();
    maxval = match maxval {
      None => Some(thismax),
      Some(v) => Some(cmp::max(thismax, v))
    };
  }
  let largest_value = registers.values().max().unwrap();
  println!("The largest value in any register is {}", largest_value);
  println!("The largest value ever to appear in any register is {}", maxval.unwrap());
  
}