extern crate aoc2017;
use aoc2017::puzzles::day16;

use std::io;
use std::io::BufRead;

fn main() {
  let stdin = io::stdin();
  
  let programs : Vec<char> = "abcdefghijklmnop".chars().collect();
  
  let steps : Vec<String> = stdin.lock().split(',' as u8).map(|s| String::from_utf8(s.unwrap()).unwrap()).collect();
  
  {
    let mut p1 : Vec<char> = programs.iter().cloned().collect();
    day16::dance(&mut p1, &steps);
    println!("Part 1: {}", day16::dance_string(&p1));
  }
  
  {
    println!("Part 2:");
    let mut p2 : Vec<char> = programs.iter().cloned().collect();
    let (offset, cycle) = day16::dance_cycle(&mut p2, &steps);
    println!("Cycle: {}, Offset: {}", cycle, offset);
    let ndances = ((1000000000 as u64 - (offset as u64)) % (cycle as u64)) as usize + offset;
    println!("We should dance {} times", ndances);
    
    let mut p3 : Vec<char> = programs.iter().cloned().collect();
    for _i in 0..ndances {
      day16::dance(&mut p3, &steps);
    }
    println!("Final arangement: {}", day16::dance_string(&p3));
    
  }
}
