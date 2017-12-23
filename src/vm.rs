/// Registers

use std::collections::HashMap;
use std::str;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Registers<'k, T>
where
    T: Copy + Clone,
{
    values: HashMap<&'k str, T>,
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

impl<'k, T> Registers<'k, T>
where
    T: str::FromStr + Copy + Clone,
{
    pub fn new(default: T) -> Registers<'k, T> {
        Registers {
            values: HashMap::new(),
            default: default,
        }
    }
    pub fn hmap(&self) -> &HashMap<&'k str, T> {
      return &self.values
    }
    
    pub fn get_mut<>(&mut self, argument: &Argument<'k, T>) -> Option<&mut T> {
        match argument {
            &Argument::Register(ref s) => Some(self.values.entry(s).or_insert(self.default)),
            &Argument::Value(_) => None,
        }
    }

    pub fn get<'a>(&'a self, argument: &'k Argument<'k, T>) -> T {
        match *argument {
            Argument::Register(ref s) => *self.values.get(s).unwrap_or(&self.default),
            Argument::Value(v) => v,
        }
    }
}


#[cfg(test)]
mod test {
  
  use super::*;
  
  #[test]
  fn test_parse_arguments() {
    
    let arg : Argument<i32> = Argument::parse("a");
    assert_eq!(arg, Argument::Register("a"));
    
    let arg : Argument<i32> = Argument::parse("-10");
    assert_eq!(arg, Argument::Value(-10));
  }
  
  #[test]
  fn test_read_registry() {
    let registry : Registers<i32> = Registers::new(0);
    let arg : Argument<i32> = Argument::parse("a");
    assert_eq!(registry.get(&arg), 0);
  }
  
  fn execute_command<'b, 'a: 'b + 'c, 'c>(registry: &mut Registers<'b, i32>, condition: &'a Argument<'a, i32>, destination: &Argument<'a, i32>) {
    if registry.get(condition) > 0 {
      let dest = registry.get_mut(destination).unwrap();
      *dest += 1;
    }
  }
  
  #[test]
  fn increment_registry() {
    let cond : Argument<i32> = Argument::parse("5");
    let dest : Argument<i32> = Argument::parse("a");
    let mut registry : Registers<i32> = Registers::new(0);
    execute_command(&mut registry, &cond, &dest);
    assert_eq!(registry.get(&Argument::Register("a")), 1);
  }
}
