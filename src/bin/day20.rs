extern crate aoc2017;
use aoc2017::puzzles::day20;
use std::io;
use std::io::BufRead;

fn main() {
  let stdin = io::stdin();

  let particles: Vec<day20::Particle> = stdin
    .lock()
    .lines()
    .map(|line| day20::Particle::parse(&line.unwrap()).unwrap())
    .collect();

  let pid = day20::find_closest(particles.clone(), false);
  println!("Particle {} will stay closest to the origin.", pid);

  let (_n, ps) = day20::simulate(particles.clone(), None, true)
    .skip(100)
    .skip_while(|&(_i, ref ps)| ps.iter().all(|x| x.settled()))
    .nth(100)
    .unwrap();
  println!("{} Particles remain after stability is achieved.", ps.len());
}
