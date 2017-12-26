/// Registers

use std::collections::HashMap;
use std::str;


// Registers key on String so that their keys
// are not tied in lifetime to the arguments
// which create the keys.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Registers<T>
where
  T: Copy + Clone,
{
  values: HashMap<String, T>,
  default: T,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Argument<'k, T>
where
  T: Copy + Clone,
{
  Register(&'k str),
  Value(T),
}

impl<'k, T> Argument<'k, T>
where
  T: str::FromStr + Copy + Clone,
{
  pub fn parse(text: &'k str) -> Argument<'k, T> {
    match text.trim().parse::<T>() {
      Ok(value) => Argument::Value(value),
      Err(_) => Argument::Register(text.trim()),
    }
  }
}

impl<T> Registers<T>
where
  T: str::FromStr + Copy + Clone,
{
  pub fn new(default: T) -> Registers<T> {
    Registers {
      values: HashMap::new(),
      default: default,
    }
  }
  pub fn hmap(&self) -> &HashMap<String, T> {
    return &self.values;
  }

  pub fn get_mut(&mut self, argument: &Argument<T>) -> Option<&mut T> {
    match argument {
      &Argument::Register(s) => Some(self.values.entry(s.to_string()).or_insert(self.default)),
      &Argument::Value(_) => None,
    }
  }

  pub fn get(&self, argument: &Argument<T>) -> T {
    match *argument {
      Argument::Register(s) => *self.values.get(s).unwrap_or(&self.default),
      Argument::Value(v) => v,
    }
  }
}


#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_parse_arguments() {
    let arg: Argument<i32> = Argument::parse("a");
    assert_eq!(arg, Argument::Register("a"));

    let arg: Argument<i32> = Argument::parse("-10");
    assert_eq!(arg, Argument::Value(-10));
  }

  #[test]
  fn test_read_registry() {
    let registry: Registers<i32> = Registers::new(0);
    let arg: Argument<i32> = Argument::parse("a");
    assert_eq!(registry.get(&arg), 0);
  }

  fn execute_command(
    registry: &mut Registers<i32>,
    condition: &Argument<i32>,
    destination: &Argument<i32>,
  ) {
    if registry.get(condition) > 0 {
      let dest = registry.get_mut(destination).unwrap();
      *dest += 1;
    }
  }

  #[test]
  fn increment_registry() {
    let cond: Argument<i32> = Argument::parse("5");
    let dest: Argument<i32> = Argument::parse("a");
    let mut registry: Registers<i32> = Registers::new(0);
    execute_command(&mut registry, &cond, &dest);
    assert_eq!(registry.get(&Argument::Register("a")), 1);
  }
}
