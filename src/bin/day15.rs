extern crate aoc2017;
use aoc2017::puzzles::day15;

fn main() {
  let mut a = day15::Generator::new(16807, 699);
  let mut b = day15::Generator::new(48271, 124);

  println!(
    "Part 1 Score: {}",
    day15::compete(&mut a, &mut b, 40e+6 as usize)
  );

  let mut a = day15::Generator::new(16807, 699);
  let mut b = day15::Generator::new(48271, 124);
  a.picky(4);
  b.picky(8);
  println!(
    "Part 2 Score: {}",
    day15::compete(&mut a, &mut b, 5e+6 as usize)
  );
}
