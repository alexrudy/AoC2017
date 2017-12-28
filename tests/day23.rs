#![feature(iterator_step_by)]
extern crate aoc2017;
use aoc2017::puzzles::day23;
use aoc2017::vm;

use std::fs;
use std::io::Read;

fn get_program<'a>(buffer: &'a mut String) -> Vec<day23::Command<'a>> {
  let mut f = fs::File::open("./puzzles/23/input.txt").unwrap();
  f.read_to_string(buffer).expect("Read failure!");
  buffer
    .lines()
    .map(|x| x.into())
    .collect::<Vec<day23::Command>>()
}

#[test]
fn part_one() {
  let mut buffer = String::new();
  let program = get_program(&mut buffer);
  let mut cpu = day23::Processor::new();

  assert_eq!(cpu.run(&program).map(|x| x as usize).sum::<usize>(), 8281);
}

#[test]
fn part_two() {
  let mut buffer = String::new();
  let program = get_program(&mut buffer);
  let mut cpu = day23::Processor::new();
  *cpu.registers.get_mut(&vm::Argument::Register("a")).unwrap() = 1;
  cpu.run(&program).nth(6).unwrap();
  assert_eq!(
    day23::decompiled_part_two(
      cpu.registers.get(&vm::Argument::Register("b")),
      cpu.registers.get(&vm::Argument::Register("c"))
    ),
    911
  );
}
