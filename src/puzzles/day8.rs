use std::collections::HashMap;
use std::io;

pub type Registers = HashMap<String, i32>;

fn parser_error(text: String) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, text)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Command {
  Increment,
  Decrement
}

impl Command {
  fn execute(&self, target: &mut i32, value: &i32) {
    match *self {
      Command::Increment => { *target += *value },
      Command::Decrement => { *target -= *value }
    }
  }
  
  fn parse(text: &str) -> Result<Command, io::Error> {
    match text {
      "inc" => Ok(Command::Increment),
      "dec" => Ok(Command::Decrement),
      _ => Err(parser_error(format!("Can't parse {}", text)))
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Argument {
  Register(String),
  Value(i32)
}


impl Argument {
  
  fn get_mut<'a>(&self, registers: &'a mut Registers) -> Option<&'a mut i32> {
    match *self {
      Argument::Register(ref s) => Some(registers.entry(s.clone()).or_insert(0)),
      Argument::Value(_) => None,
    }
  }
  
  fn get(&self, registers: &Registers) -> Option<i32> {
    match *self {
      Argument::Register(ref s) => Some(*registers.get(s).unwrap_or(&0)),
      Argument::Value(v) => Some(v)
    }
  }
  
  fn parse(text: &str) -> Result<Argument, io::Error> {
    match text.parse::<i32>() {
      Ok(value) => Ok(Argument::Value(value)),
      Err(_) => Ok(Argument::Register(text.trim().to_string())),
    }
  }
  
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operator {
  GreaterThan,
  LessThan,
  GreaterEqual,
  Equal,
  LessEqual,
  NotEqual
}

impl Operator {
  
  fn execute(&self, left: i32, right: i32) -> bool {
    match *self {
      Operator::GreaterThan => { left > right },
      Operator::LessThan => { left < right },
      Operator::GreaterEqual => { left >= right },
      Operator::Equal => { left == right },
      Operator::LessEqual => { left <= right },
      Operator::NotEqual => { left != right }
    }
  }
  
  fn parse(text: &str) -> Result<Operator, io::Error> {
    match text.trim() {
      ">" => Ok(Operator::GreaterThan),
      "<" => Ok(Operator::LessThan),
      ">=" => Ok(Operator::GreaterEqual),
      "<=" => Ok(Operator::LessEqual),
      "==" => Ok(Operator::Equal),
      "!=" => Ok(Operator::NotEqual),
      "=!" => Ok(Operator::NotEqual),
      _ => Err(parser_error(format!("Can't parse {}", text)))
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction {
  destination: Argument,
  command: Command,
  value: Argument,
  left: Argument,
  operator: Operator,
  right: Argument
}

impl Instruction {
  
  /// Check the condition for this instruction.
  fn condition(&self, registers: &mut Registers) -> bool {
    let left = self.left.get(registers).unwrap();
    let right = self.right.get(registers).unwrap();
    self.operator.execute(left, right)
  }
  
  pub fn execute(&self, registers: &mut Registers) {
    if self.condition(registers) {
      let right = self.value.get(registers).unwrap();
      let left = self.destination.get_mut(registers).unwrap();
      self.command.execute(left, &right);
    }
  }
  
  pub fn parse(text: &str) -> Result<Instruction, io::Error> {
    let mut parts = text.split_whitespace();
    let destination = Argument::parse(parts.next().unwrap())?;
    let command = Command::parse(parts.next().unwrap())?;
    let value = Argument::parse(parts.next().unwrap())?;
    
    let _ifstatement = parts.next().unwrap();
    let left = Argument::parse(parts.next().unwrap())?;
    let op = Operator::parse(parts.next().unwrap())?;
    let right = Argument::parse(parts.next().unwrap())?;
    
    Ok(Instruction {
      destination: destination,
      command: command,
      value: value,
      left: left,
      operator: op,
      right: right,
    })
  }
  
}

#[cfg(test)]
mod test {
  
  use super::*;
  use std::io::BufRead;
  
  #[test]
  fn parse_arguments() {
    
    // Arguments
    let arg = Argument::parse("a").unwrap();
    assert_eq!(arg, Argument::Register("a".to_string()));
    
    let arg = Argument::parse("-10").unwrap();
    assert_eq!(arg, Argument::Value(-10));
  }
  
  #[test]
  fn parse_command() {
    // Commands
    let com = Command::parse("inc").unwrap();
    assert_eq!(com, Command::Increment);
    
    let com = Command::parse("dec").unwrap();
    assert_eq!(com, Command::Decrement);
    
    let com = Command::parse("foo");
    assert!(com.is_err());
  }
  
  #[test]
  fn parse_operator() {
    let op = Operator::parse(">").unwrap();
    assert_eq!(op, Operator::GreaterThan);
    
    let op = Operator::parse("<").unwrap();
    assert_eq!(op, Operator::LessThan);
  }
  
  #[test]
  fn parse_instruction() {
    let mut registers = Registers::new();
    let text = "a inc 10 if b < 5";
    let instruction = Instruction::parse(text).unwrap();
    instruction.execute(&mut registers);
    assert_eq!(registers.get("a").unwrap(), &10);
  }
  
  #[test]
  fn execute_instruction() {
    let mut registers = Registers::new();
    let instruction = Instruction {
      destination: Argument::Register("a".to_string()),
      command: Command::Increment,
      value: Argument::Value(10),
      left: Argument::Register("b".to_string()),
      operator: Operator::LessThan,
      right: Argument::Value(5)
    };
    instruction.execute(&mut registers);
    assert_eq!(registers.get("a").unwrap(), &10);
  }
  
  #[test]
  fn execute_program() {
    let program = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
    let mut registers = Registers::new();
    for statement in program.as_bytes().lines() {
      let instruction = Instruction::parse(&statement.unwrap()).unwrap();
      instruction.execute(&mut registers);
    }
    assert_eq!(registers.values().max(), Some(&1));
  }
}