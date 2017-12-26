extern crate aoc2017;
use aoc2017::puzzles::day12;
use aoc2017::graph;
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
  let stdlock = io::stdin();
  let mut g: graph::Graph<day12::Town> = graph::Graph::new();

  for line in stdlock.lock().lines() {
    g.parse_node(&line.unwrap()).unwrap();
  }

  let root = g.find_node("0").unwrap();
  println!(
    "Number of nodes connected to 0: {}",
    root.connected(&g).count()
  );

  let mut ngroups = 0;
  let mut seen: HashSet<graph::Node> = HashSet::new();
  for node in g.iter() {
    if !seen.contains(&node) {
      ngroups += 1;
      seen.insert(node);
      for cnode in node.connected(&g) {
        seen.insert(cnode);
      }
    }
  }

  println!("There are {} groups.", ngroups);
}
