use super::super::vm;
use std::str;

#[allow(non_camel_case_types)]
type int = isize;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Progression {
  step: isize,
  sound: Option<int>,
}

impl Progression {
  fn new(step: isize, sound: Option<int>) -> Progression {
    Progression { step, sound }
  }
  
  fn step() -> Progression {
    Progression{ step:1, sound:None }
  }
  
  fn jump(step: isize) -> Progression {
    Progression{ step:step, sound:None }
  }
  
  fn sound(&self) -> Option<int> {
    self.sound
  }
  
  fn get(&self) -> isize {
    self.step
  }
}

/// A structure to hold pairs of arguments
/// for commands which accept two arguments.
#[derive(Debug, PartialEq, Eq, Clone)]
struct Arguments<'a, T> 
  where T: Copy + Clone + str::FromStr
{
  target: vm::Argument<'a, T>,
  argument: vm::Argument<'a, T>,
}

impl<'a, T> Arguments<'a, T>
  where T: Copy + Clone + str::FromStr
{
  fn parse<Q>(text: &mut Q) -> Arguments<'a, T> 
    where Q: Iterator<Item=&'a str>
  {
    Arguments { 
      target: vm::Argument::parse(text.next().expect("Requires two arguments!")),
      argument: vm::Argument::parse(text.next().expect("Requires two arguments!"))
    }
  }
  
}

/// The VM Commands
#[derive(Debug, PartialEq, Eq, Clone)]
enum Command<'a> {
  Snd(vm::Argument<'a, int>),
  Set(Arguments<'a, int>),
  Add(Arguments<'a, int>),
  Mul(Arguments<'a, int>),
  Mod(Arguments<'a, int>),
  Rcv(vm::Argument<'a, int>),
  Jgz(Arguments<'a, int>)
}

impl<'a> Command<'a> {
  
  fn parse(text: &'a str) -> Command<'a> {
    let mut parts = text.split_whitespace();
    match parts.next().expect("Need to find a command!") {
      "snd" => Command::Snd(vm::Argument::parse(parts.next().expect("Requires one argument"))),
      "set" => Command::Set(Arguments::parse(&mut parts)),
      "add" => Command::Add(Arguments::parse(&mut parts)),
      "mul" => Command::Mul(Arguments::parse(&mut parts)),
      "mod" => Command::Mod(Arguments::parse(&mut parts)),
      "rcv" => Command::Rcv(vm::Argument::parse(parts.next().expect("Requires one argument"))),
      "jgz" => Command::Jgz(Arguments::parse(&mut parts)),
      _ => panic!("Can't understand command!")
    }
  }
  
}

/// The actual computer component
#[derive(Debug, PartialEq, Eq, Clone)]
struct Transmitter {
  registers: vm::Registers<int>,
  sound: Option<int>
}

impl Transmitter {
  
  fn new() -> Transmitter {
    Transmitter {
      registers: vm::Registers::new(0),
      sound: None
    }
  }
  
  fn snd(&mut self, arg: &vm::Argument<int>) -> Progression {
    let value = self.registers.get(arg);
    self.sound = Some(value);
    Progression::step()
  }
  
  fn set(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self.registers.get_mut(&args.target).expect("Requires a register!");
    *target = value;
    Progression::step()
  }
  
  fn add(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self.registers.get_mut(&args.target).expect("Requires a register!");
    *target += value;
    Progression::step()
  }
  
  fn mul(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self.registers.get_mut(&args.target).expect("Requires a register!");
    *target *= value;
    Progression::step()
  }
  
  fn mod_(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self.registers.get_mut(&args.target).expect("Requires a register!");
    *target = *target % value;
    Progression::step()
  }
  
  fn rcv(&self, arg: &vm::Argument<int>) -> Progression {
    if self.registers.get(arg) > 0 {
      Progression::new(1, self.sound)
    } else {
      Progression::step()
    }
  }
  
  fn jgz(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self.registers.get(&args.target);
    if target > 0 {
      Progression::jump(value)
    } else {
      Progression::step()
    }
  }
  
  /// Execute the command, and return the offset for the next position
  fn execute(&mut self, cmd: &Command) -> Progression {
    match *cmd {
      Command::Snd(ref arg) => {  self.snd(arg) },
      Command::Set(ref args) => { self.set(args) },
      Command::Add(ref args) => { self.add(args) },
      Command::Mul(ref args) => { self.mul(args) },
      Command::Mod(ref args) => { self.mod_(args) },
      Command::Rcv(ref arg) => { self.rcv(arg) },
      Command::Jgz(ref arg) => { self.jgz(arg) },
    }
    
  }
  
}

pub struct TransmissionIterator<'a> {
  commands: Vec<Command<'a>>,
  transmitter: Transmitter,
  position: Option<isize>
}

impl<'a> Iterator for TransmissionIterator<'a> {
  type Item = int;
  fn next(&mut self) -> Option<int> {
    
    // Program has not expired, iterate:
    if let Some(pos) = self.position {
      
      // Step through progress until we are 
      let mut nextpos = pos;
      loop {
        
        // Step to the next position
        let progress = self.transmitter.execute(&self.commands[nextpos as usize]);
        nextpos += progress.get();
        
        // We've ended the program, 
        // note this fact, then return the sound
        if !((0 <= nextpos) & (nextpos < self.commands.len() as isize)) {
          self.position = None;
          return progress.sound();
        }
        
        // Check if we should yield a sound
        if progress.sound().is_some() {
          self.position = Some(nextpos);
          return progress.sound();
        }
      }
    } else {
      // Program has expired, signal that we are done.
      return None;
    }
  }
}

pub fn run_program<'a>(program: &'a str) -> TransmissionIterator<'a> {
  let commands = program.lines().map(|x| Command::parse(x)).collect();
  TransmissionIterator {
    commands: commands,
    transmitter: Transmitter::new(),
    position: Some(0),
  }
}

#[cfg(test)]
mod test {
  
  use super::*;
  
  #[test]
  fn test_parse_commands() {
    let cmd = Command::parse("snd a");
    assert_eq!(cmd, Command::Snd(vm::Argument::Register("a")));
    
    let cmd = Command::parse("set a 10");
    let args = Arguments { target: vm::Argument::Register("a"), argument: vm::Argument::Value(10) };
    assert_eq!(cmd, Command::Set(args));
  }
  
  #[test]
  fn test_execute_commands() {
    let mut transmitter = Transmitter::new();
    let p = transmitter.execute(&Command::parse("snd a"));
    assert_eq!(p, Progression::new(1, None));
    let p = transmitter.execute(&Command::parse("set a 10"));
    assert_eq!(p, Progression::new(1, None));
    let p = transmitter.execute(&Command::parse("rcv 1"));
    assert_eq!(p, Progression::new(1, Some(0)));
    let p = transmitter.execute(&Command::parse("add a 1"));
    assert_eq!(p, Progression::new(1, None));
    let p = transmitter.execute(&Command::parse("mul b 2"));
    assert_eq!(p, Progression::new(1, None));
    let p = transmitter.execute(&Command::parse("mod a 10"));
    assert_eq!(p, Progression::new(1, None));
    let p = transmitter.execute(&Command::parse("jgz a 10"));
    assert_eq!(p, Progression::new(10, None));
    let p = transmitter.execute(&Command::parse("snd a"));
    assert_eq!(p, Progression::new(1, None));
    let p = transmitter.execute(&Command::parse("rcv 1"));
    assert_eq!(p, Progression::new(1, Some(1)));
  }
  
  #[test]
  fn test_execute_program () {
    let program = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
    assert_eq!(run_program(&program).take(1).next(), Some(4));
  }
  
}
