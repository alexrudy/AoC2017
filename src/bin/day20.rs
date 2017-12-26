extern crate aoc2017;
use aoc2017::puzzles::day20;
use std::io;
use std::io::BufRead;

fn main() {
  let stdin = io::stdin();

  let particles = stdin
    .lock()
    .lines()
    .map(|line| day20::Particle::parse(&line.unwrap()).unwrap())
    .collect();

  let (_n, pf) = day20::simulate(
    particles,
    Some(&|p: &[day20::Particle]| -> bool {
      p.iter().all(|pi| pi.settled())
    }),
  ).nth(0)
    .unwrap();
  let (id, _pm) = pf.iter()
    .enumerate()
    .min_by_key(|&(_i, pi)| pi.distance())
    .unwrap();
  println!("Particle {} will stay closest to the origin.", id);
}
