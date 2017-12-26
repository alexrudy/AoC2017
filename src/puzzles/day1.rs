pub fn robot_captcha(numbers: &[u32]) -> u32 {
  let mut sum: u32 = 0;
  let mut previous_value = numbers.last().expect("There must be at least one element.");
  for value in numbers.iter() {
    if value == previous_value {
      sum += *value
    }
    previous_value = value
  }
  return sum;
}

pub fn robot_captcha_half(numbers: &[u32]) -> u32 {
  let mut sum: u32 = 0;
  let halfway = numbers.len() / 2;
  let mut halfway_value: &u32;
  for (idx, value) in numbers.iter().enumerate() {
    if idx + halfway >= halfway * 2 {
      halfway_value = &numbers[idx - halfway]
    } else {
      halfway_value = &numbers[idx + halfway]
    }
    if value == halfway_value {
      sum += *value
    }
  }
  return sum;
}

pub fn parse_captcha(captcha: String, result: &mut Vec<u32>) {
  for chr in captcha.trim().chars() {
    result.push(chr.to_digit(10).expect("Can't parse digit."))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let numbers = vec![1, 1, 2, 2];
    assert_eq!(robot_captcha(&numbers), 3);
  }

  #[test]
  fn example_2() {
    let numbers = vec![1, 1, 1, 1];
    assert_eq!(robot_captcha(&numbers), 4);
  }

  #[test]
  fn example_3() {
    let numbers = vec![1, 2, 3, 4];
    assert_eq!(robot_captcha(&numbers), 0);
  }

  #[test]
  fn example_4() {
    let numbers = vec![9, 1, 2, 1, 2, 1, 2, 9];
    assert_eq!(robot_captcha(&numbers), 9);
  }

  #[test]
  fn example_half_1() {
    let numbers = vec![1, 2, 1, 2];
    assert_eq!(robot_captcha_half(&numbers), 6);
  }

  #[test]
  fn example_half_2() {
    let numbers = vec![1, 2, 2, 1];
    assert_eq!(robot_captcha_half(&numbers), 0);
  }

  #[test]
  fn example_half_3() {
    let numbers = vec![1, 2, 3, 4, 2, 5];
    assert_eq!(robot_captcha_half(&numbers), 4);
  }

  #[test]
  fn example_half_4() {
    let numbers = vec![1, 2, 3, 1, 2, 3];
    assert_eq!(robot_captcha_half(&numbers), 12);
  }

  #[test]
  fn example_half_5() {
    let numbers = vec![1, 2, 1, 3, 1, 4, 1, 5];
    assert_eq!(robot_captcha_half(&numbers), 4);
  }

  #[test]
  fn parsing() {
    let input_text = "91212129".to_string();
    let mut numbers: Vec<u32> = Vec::new();
    parse_captcha(input_text, &mut numbers);
    assert_eq!(numbers, vec![9, 1, 2, 1, 2, 1, 2, 9]);
    assert_eq!(robot_captcha(&numbers), 9)
  }
}
