extern crate aoc2017;
use aoc2017::puzzles::day7;
use aoc2017::graph;
use std::io;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  
  let mut programs : graph::Graph<day7::Program> = graph::Graph::new();
  programs.parse_nodes(stdin.lock().lines()).unwrap();
  
  let root = programs.find_root().map(|x| programs.get_data(&x)).unwrap();
  println!("Root node is: {}", root.name);
  
  for node in programs.iter() {
    if node.badweight(&programs) {
      println!("Fixed weight is {} -> {}", 
        programs.get_data(&node).weight,
        node.fixed_weight(&programs).unwrap())
    }
    
  }
}