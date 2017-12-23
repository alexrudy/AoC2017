use std::collections::HashMap;
use std::io;

use super::super::vm;

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
  fn execute(&self, target: &mut i32, value: i32) {
    match *self {
      Command::Increment => { *target += value },
      Command::Decrement => { *target -= value }
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
pub struct Instruction<'a> {
  destination: vm::Argument<'a, i32>,
  command: Command,
  value: vm::Argument<'a, i32>,
  left: vm::Argument<'a, i32>,
  operator: Operator,
  right: vm::Argument<'a, i32>,
}

impl<'a> Instruction<'a> {
  
  /// Check the condition for this instruction.
  fn condition(&self, registers: &vm::Registers<i32>) -> bool {
    let left = registers.get(&self.left);
    let right = registers.get(&self.right);
    self.operator.execute(left, right)
  }
  
  pub fn execute(&self, registers: &mut vm::Registers<i32>) {
    if self.condition(registers) {
      let right = registers.get(&self.value);
      let left = registers.get_mut(&self.destination).unwrap();
      self.command.execute(left, right);
    }
  }
  
  pub fn parse(text: &str) -> Result<Instruction, io::Error> {
    let mut parts = text.split_whitespace();
    let destination = vm::Argument::parse(parts.next().unwrap());
    let command = Command::parse(parts.next().unwrap())?;
    let value = vm::Argument::parse(parts.next().unwrap());
    
    let _ifstatement = parts.next().unwrap();
    let left = vm::Argument::parse(parts.next().unwrap());
    let op = Operator::parse(parts.next().unwrap())?;
    let right = vm::Argument::parse(parts.next().unwrap());
    
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
    let arg : vm::Argument<i32> = vm::Argument::parse("a");
    assert_eq!(arg, vm::Argument::Register("a"));
    
    let arg : vm::Argument<i32> = vm::Argument::parse("-10");
    assert_eq!(arg, vm::Argument::Value(-10));
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
    let mut registers = vm::Registers::new(0);
    let text = "a inc 10 if b < 5";
    let instruction = Instruction::parse(text).unwrap();
    instruction.execute(&mut registers);
    assert_eq!(registers.hmap().get("a").unwrap(), &10);
  }
  
  #[test]
  fn execute_instruction() {
    let mut registers = vm::Registers::new(0);
    {
    let instruction = Instruction {
      destination: vm::Argument::Register("a"),
      command: Command::Increment,
      value: vm::Argument::Value(10),
      left: vm::Argument::Register("b"),
      operator: Operator::LessThan,
      right: vm::Argument::Value(5)
    };
    instruction.execute(&mut registers);
    }
    assert_eq!(registers.hmap().get("a").unwrap(), &10);
  }
  
  #[test]
  fn execute_program() {
    let program = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
    let mut registers = vm::Registers::new(0);
    for statement in program.as_bytes().lines() {
      let line = statement.unwrap();
      let instruction = Instruction::parse(&line).unwrap();
      instruction.execute(&mut registers);
    }
    assert_eq!(registers.hmap().values().max(), Some(&1));
  }
}