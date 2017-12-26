use super::super::knot;

pub fn part1(raw_input: &str) -> u32 {
  let input: Vec<usize> = raw_input
    .split(',')
    .map(|x| x.parse::<usize>().unwrap())
    .collect();

  let mut knot = knot::Knot::new(256);
  knot.compute(&input, 1);
  knot.simpledigest()
}

pub fn part2(raw_input: &str) -> knot::Knot {
  knot::Knot::standard(&raw_input)
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn try_hashes() {
    assert_eq!(
      &knot::Knot::standard("AoC 2017").hexdigest(),
      "33efeb34ea91902bb2f59c9920caa6cd"
    );
    assert_eq!(
      &knot::Knot::standard("1,2,3").hexdigest(),
      "3efbe78a8d82f29979031a4aa0b16a9d"
    );
    assert_eq!(
      &knot::Knot::standard("1,2,4").hexdigest(),
      "63960835bcdc130f0b66d7ff4f6a5a8e"
    );
    assert_eq!(
      &knot::Knot::standard("").hexdigest(),
      "a2582a3a0e66e6e86e3812dcb672a272"
    );
  }

}
