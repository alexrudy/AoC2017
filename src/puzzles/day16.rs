use std::collections::HashMap;

/// Perform a dance move in place.
pub fn dance_move(dancers: &mut [char], command: &str) {
  match command.chars().nth(0).unwrap() {
    's' => dance_spin(dancers, command),
    'x' => dance_exchange(dancers, command),
    'p' => dance_partner(dancers, command),
    _ => panic!("I don't know this command!"),
  }
}

/// Perform the spin move, rotating n positions.
fn dance_spin(dancers: &mut [char], command: &str) {
  let spin = dancers.len() - command[1..].parse::<usize>().unwrap() % dancers.len();
  dancers.rotate(spin);
}

/// Perform the exchange move, switching two indicies.
fn dance_exchange(dancers: &mut [char], command: &str) {
  let mut moves = command[1..]
    .split('/')
    .map(|x| x.trim().parse::<usize>().unwrap());
  let a = moves.next().unwrap();
  let b = moves.next().unwrap();
  dancers.swap(a, b);
}

/// Perform the partner move, switching two labels.
fn dance_partner(dancers: &mut [char], command: &str) {
  let mut partners = command[1..]
    .split('/')
    .map(|x| x.trim().parse::<char>().unwrap());
  let a = partners.next().unwrap();
  let b = partners.next().unwrap();

  let ia = dancers
    .iter()
    .enumerate()
    .find(|&(_i, x)| *x == a)
    .unwrap()
    .0;
  let ib = dancers
    .iter()
    .enumerate()
    .find(|&(_i, x)| *x == b)
    .unwrap()
    .0;
  dancers.swap(ia, ib);
}

pub fn dance(dancers: &mut [char], steps: &[String]) {
  for step in steps {
    dance_move(dancers, step);
  }
}

pub fn dance_string(dancers: &[char]) -> String {
  dancers.iter().cloned().collect::<String>()
}

pub fn dance_cycle(dancers: &mut [char], steps: &[String]) -> (usize, usize) {
  let mut seen: HashMap<String, usize> = HashMap::new();
  let mut dstring = dance_string(dancers);

  for i in 0.. {
    seen.insert(dstring.clone(), i);
    dance(dancers, steps);
    dstring = dance_string(dancers);
    if seen.contains_key(&dstring) {
      let x = *seen.get(&dstring).unwrap();
      return (x, i - x + 1);
    }
  }
  panic!("How did we get here?")
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn simple_dance() {
    let mut programs: Vec<char> = "abcde".chars().collect();
    let steps: Vec<String> = vec!["s1", "x3/4", "pe/b"]
      .iter()
      .map(|x| x.to_string())
      .collect();
    println!("Dancing...");
    println!(" ) {}", programs.iter().cloned().collect::<String>());

    for (i, step) in steps.iter().enumerate() {
      dance_move(&mut programs, step);
      println!("{}) {}", i, programs.iter().cloned().collect::<String>());
    }

    let s: String = programs.iter().cloned().collect();
    assert_eq!(&s, "baedc");
  }

  #[test]
  fn find_cycle() {
    let mut programs: Vec<char> = "abcde".chars().collect();
    let steps: Vec<String> = vec!["s1", "x3/4", "pe/b"]
      .iter()
      .map(|x| x.to_string())
      .collect();
    let (offset, cycle) = dance_cycle(&mut programs, &steps);
    assert_eq!(offset, 0);
    assert_eq!(cycle, 4);

    programs = "abcde".chars().collect();
    for _i in 0..offset {
      dance(&mut programs, &steps);
    }
    let or = dance_string(&programs);
    for _i in 0..cycle {
      dance(&mut programs, &steps);
    }
    assert_eq!(or, dance_string(&programs))
  }

  use test::Bencher;
  use std::fs::File;
  use std::io::prelude::*;
  use std::io::BufReader;

  #[bench]
  fn bench_dance(b: &mut Bencher) {
    let f = File::open("puzzles/16/input.txt").expect("file not found");
    let reader = BufReader::new(f);
    let steps: Vec<String> = reader
      .split(',' as u8)
      .map(|s| String::from_utf8(s.unwrap()).unwrap())
      .collect();
    let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
    b.iter(|| {
      dance(&mut programs, &steps);
    });
  }
}
