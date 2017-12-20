extern crate aoc2017;
use aoc2017::puzzles::day17;

fn main() {
  let input = 382;
  let locked = day17::spinlock(2018, input);
  println!("Part 1: {}", day17::getnextitem(&locked, 2017));
  
  println!("Part 2: {}", day17::spinlock_afterzero(50000001, input));
  
  
}
