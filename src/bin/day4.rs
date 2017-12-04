extern crate aoc2017;
use aoc2017::puzzles::day4;
use std::io;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  let mut nphrase = 0;
  let mut nphrase_anagram = 0;
  let _ : u32 = stdin.lock().lines().map(|l| {
    let line = l.unwrap();
    if day4::check_passphrase(&line) { nphrase += 1 };
    if day4::check_passphrase_anagrams(&line) { nphrase_anagram += 1 }; 1}).sum();
  
  println!("There are {} valid passphrases.", nphrase);
  println!("There are {} valid passphrases without anagrams.", nphrase_anagram);
  
  
  
}