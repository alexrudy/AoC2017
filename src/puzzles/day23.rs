use super::super::vm;

type Progression = Result<isize, String>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command<'a> {
  Set(vm::Argument<'a, isize>, vm::Argument<'a, isize>),
  Sub(vm::Argument<'a, isize>, vm::Argument<'a, isize>),
  Mul(vm::Argument<'a, isize>, vm::Argument<'a, isize>),
  Jnz(vm::Argument<'a, isize>, vm::Argument<'a, isize>),
}

impl<'a> From<&'a str> for Command<'a> {
  fn from(text: &'a str) -> Self {
    let mut parts = text.split_whitespace();
    match parts.next().expect("Need to find a command!") {
      "set" => Command::Set(parts.next().unwrap().into(), parts.next().unwrap().into()),
      "sub" => Command::Sub(parts.next().unwrap().into(), parts.next().unwrap().into()),
      "mul" => Command::Mul(parts.next().unwrap().into(), parts.next().unwrap().into()),
      "jnz" => Command::Jnz(parts.next().unwrap().into(), parts.next().unwrap().into()),
      _ => panic!("Can't understand command!"),
    }
  }
}

impl<'a> Command<'a> {
  fn set(
    &self,
    registers: &mut vm::Registers<isize>,
    args: (&vm::Argument<isize>, &vm::Argument<isize>),
  ) -> Progression {
    let value = registers.get(&args.1);
    let target = registers.get_mut(&args.0).expect("Requires a register!");
    *target = value;
    Ok(1)
  }

  fn sub(
    &self,
    registers: &mut vm::Registers<isize>,
    args: (&vm::Argument<isize>, &vm::Argument<isize>),
  ) -> Progression {
    let value = registers.get(&args.1);
    let target = registers.get_mut(&args.0).expect("Requires a register!");
    *target -= value;
    Ok(1)
  }

  fn mul(
    &self,
    registers: &mut vm::Registers<isize>,
    args: (&vm::Argument<isize>, &vm::Argument<isize>),
  ) -> Progression {
    let value = registers.get(&args.1);
    let target = registers.get_mut(&args.0).expect("Requires a register!");
    *target *= value;
    Ok(1)
  }

  fn jnz(
    &self,
    registers: &mut vm::Registers<isize>,
    args: (&vm::Argument<isize>, &vm::Argument<isize>),
  ) -> Progression {
    let value = registers.get(&args.1);
    let target = registers.get(&args.0);
    if target != 0 {
      Ok(value)
    } else {
      Ok(1)
    }
  }
}

pub struct Processor {
  pub registers: vm::Registers<isize>,
}

impl Processor {
  pub fn new() -> Self {
    Processor {
      registers: vm::Registers::new(0),
    }
  }

  fn execute(&mut self, cmd: &Command) -> Progression {
    match cmd {
      &Command::Set(ref target, ref value) => cmd.set(&mut self.registers, (target, value)),
      &Command::Sub(ref target, ref value) => cmd.sub(&mut self.registers, (target, value)),
      &Command::Mul(ref target, ref value) => cmd.mul(&mut self.registers, (target, value)),
      &Command::Jnz(ref target, ref value) => cmd.jnz(&mut self.registers, (target, value)),
    }
  }

  pub fn run<'a>(&'a mut self, program: &'a [Command<'a>]) -> ProcessIteartor<'a> {
    ProcessIteartor {
      processor: self,
      program: program,
      position: Some(0),
    }
  }
}

pub struct ProcessIteartor<'a> {
  processor: &'a mut Processor,
  program: &'a [Command<'a>],
  position: Option<usize>,
}

impl<'a> Iterator for ProcessIteartor<'a> {
  type Item = bool;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(ind) = self.position {
      let nextcmd = &self.program[ind];
      let result = match nextcmd {
        &Command::Mul(_, _) => true,
        _ => false,
      };
      match self.processor.execute(nextcmd) {
        Ok(progress) => {
          let nextpos = (ind as isize) + progress;

          // Have we run off the end of things?
          if !((0 <= nextpos) & (nextpos < self.program.len() as isize)) {
            self.position = None;
          } else {
            self.position = Some(nextpos as usize);
          }
          Some(result)
        }
        Err(_) => {
          self.position = None;
          Some(result)
        }
      }
    } else {
      None
    }
  }
}

pub fn decompiled_part_two(b: isize, c: isize) -> usize {
  (b..(c + 1))
    .step_by(17)
    .map(|bi| (2..bi / 2).any(|d| bi % d == 0) as usize)
    .sum()
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_execute_program() {
    let program = "set a 10
set b 2
sub a 1
mul b a
jnz a -2"
      .lines()
      .map(|l| l.into())
      .collect::<Vec<Command>>();
    let mut cpu = Processor::new();

    assert_eq!(cpu.run(&program).map(|x| x as usize).sum::<usize>(), 10);
  }

  #[test]
  fn test_parse_commands() {
    let cmd: Command = "mul a 2".into();
    assert_eq!(
      cmd,
      Command::Mul(vm::Argument::Register("a"), vm::Argument::Value(2))
    );

    let cmd: Command = "set a 10".into();
    assert_eq!(
      cmd,
      Command::Set(vm::Argument::Register("a"), vm::Argument::Value(10))
    );
  }

  #[test]
  fn test_execute_commands() {
    let mut cpu = Processor::new();

    let p = cpu.execute(&"set a 10".into());
    assert_eq!(p, Ok(1));
    let p = cpu.execute(&"sub a 1".into());
    assert_eq!(p, Ok(1));
    let p = cpu.execute(&"mul b 2".into());
    assert_eq!(p, Ok(1));
    let p = cpu.execute(&"jnz a 10".into());
    assert_eq!(p, Ok(10));
  }
}
