extern crate aoc2017;
use aoc2017::puzzles::day12;
use aoc2017::graph;

use std::io::BufRead;

#[test]
fn part_one() {
  let input = include_bytes!("../puzzles/12/input.txt");
  let mut g: graph::Graph<day12::Town> = graph::Graph::new();

  for line in input.lines() {
    g.parse_node(&line.unwrap()).unwrap();
  }

  let root = g.find_node("0").unwrap();
  assert_eq!(root.connected(&g).count(), 169);
}

#[test]
fn part_two() {
  let input = include_bytes!("../puzzles/12/input.txt");
  let mut g: graph::Graph<day12::Town> = graph::Graph::new();

  for line in input.lines() {
    g.parse_node(&line.unwrap()).unwrap();
  }

  assert_eq!(g.count_groups(), 179);
}
