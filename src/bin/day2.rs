extern crate aoc2017;
use aoc2017::puzzles::day2;
use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
  
  // let mut spreadsheet = String::new();
  // io::stdin().read_to_string(&mut spreadsheet).unwrap();
  
  let stdin = io::stdin();
  let (checksum, divides) = day2::even_divides_and_checksum(stdin.lock().lines()).unwrap_or_else(|err| {
    eprintln!("Application Error: {}", err);
    process::exit(1);
  });
  
  // let checksum = day2::checksum(spreadsheet.as_bytes().lines()).unwrap();
  println!("Checksum: {}", checksum);
  
  // let divides = day2::even_divides(spreadsheet.as_bytes().lines()).unwrap();
  println!("Divides: {}", divides);
}