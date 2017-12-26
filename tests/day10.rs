extern crate aoc2017;
use aoc2017::knot;

#[test]
fn part_one() {
  let raw_input = "189,1,111,246,254,2,0,120,215,93,255,50,84,15,94,62";
  let input: Vec<usize> = raw_input
    .split(',')
    .map(|x| x.parse::<usize>().unwrap())
    .collect();

  let mut knot = knot::Knot::new(256);
  knot.compute(&input, 1);
  assert_eq!(knot.simpledigest(), 38415);
}
