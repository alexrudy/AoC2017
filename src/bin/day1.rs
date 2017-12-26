extern crate aoc2017;
use aoc2017::puzzles::day1;
use std::io;

fn main() {
  let mut captcha = String::new();
  io::stdin()
    .read_line(&mut captcha)
    .expect("Failed to read captcha");
  let mut numbers: Vec<u32> = Vec::new();
  day1::parse_captcha(captcha, &mut numbers);

  let result = day1::robot_captcha(&numbers);
  println!("The 1st captcha is {}", result);

  let result = day1::robot_captcha_half(&numbers);
  println!("The 2nd captcha is {}", result);
}
