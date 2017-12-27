#![feature(iterator_step_by)]
extern crate aoc2017;
use aoc2017::puzzles::day23;
use aoc2017::vm;

use std::io;
use std::io::Read;

fn main() {
  let mut raw_program = String::new();
  let stdin = io::stdin();
  stdin
    .lock()
    .read_to_string(&mut raw_program)
    .expect("Read failure!");

  {
    let program = raw_program
      .lines()
      .map(|x| x.into())
      .collect::<Vec<day23::Command>>();
    let mut cpu = day23::Processor::new();

    println!(
      "Part 1: {} mul instructions processed.",
      cpu.run(&program).map(|x| x as usize).sum::<usize>()
    );
  }
  {
    let program = raw_program
      .lines()
      .map(|x| x.into())
      .collect::<Vec<day23::Command>>();
    let mut cpu = day23::Processor::new();
    *cpu.registers.get_mut(&vm::Argument::Register("a")).unwrap() = 1;
    println!("Part 2:");
    cpu.run(&program).nth(6).unwrap();
    println!("Registers after init: {:?}", cpu.registers.hmap());
    
    println!("h={}", 
      part_two(cpu.registers.get(&vm::Argument::Register("b")), cpu.registers.get(&vm::Argument::Register("c"))));
    
  }
}

fn part_two(b:isize, c:isize) -> usize {
  (b..(c+1)).step_by(17).map(|bi| (2..bi/2).any(|d| bi % d == 0) as usize).sum()
}
