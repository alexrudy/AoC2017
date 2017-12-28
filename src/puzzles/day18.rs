use super::super::vm;
use std::str;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[allow(non_camel_case_types)]
type int = isize;

type Progression = Result<isize, String>;

/// A structure to hold pairs of arguments for commands which accept two
/// arguments.
#[derive(Debug, PartialEq, Eq, Clone)]
struct Arguments<'a, T>
where
  T: Copy + Clone + str::FromStr,
{
  target: vm::Argument<'a, T>,
  argument: vm::Argument<'a, T>,
}

impl<'a, T> Arguments<'a, T>
where
  T: Copy + Clone + str::FromStr,
{
  fn parse<Q>(text: &mut Q) -> Arguments<'a, T>
  where
    Q: Iterator<Item = &'a str>,
  {
    Arguments {
      target: text.next().expect("Requires 2 arguments, got 0").into(),
      argument: text.next().expect("Requires 2 arguments, got 1").into(),
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
  Jgz(Arguments<'a, int>),
}

impl<'a> Command<'a> {
  fn parse(text: &'a str) -> Command<'a> {
    let mut parts = text.split_whitespace();
    match parts.next().expect("Need to find a command!") {
      "snd" => Command::Snd(parts.next().expect("Requires 1 argument").into()),
      "set" => Command::Set(Arguments::parse(&mut parts)),
      "add" => Command::Add(Arguments::parse(&mut parts)),
      "mul" => Command::Mul(Arguments::parse(&mut parts)),
      "mod" => Command::Mod(Arguments::parse(&mut parts)),
      "rcv" => Command::Rcv(parts.next().expect("Requires 1 argument").into()),
      "jgz" => Command::Jgz(Arguments::parse(&mut parts)),
      _ => panic!("Can't understand command!"),
    }
  }
}

type Counter = Arc<(Mutex<int>, Condvar)>;

/// The actual computer component
#[derive(Debug)]
pub struct Transmitter {
  registers: vm::Registers<int>,
  output: Sender<int>,
  input: Receiver<int>,
  qsize: Counter, // Counter of the total queue size?
  sends: usize,
  ident: usize,
}

impl Transmitter {
  fn counter(n: int) -> Counter {
    Arc::new((Mutex::new(n), Condvar::new()))
  }

  fn single() -> (Transmitter, Sender<int>, Receiver<int>, Counter) {
    let (tx, input) = channel();
    let (output, rx) = channel();
    let qsize = Transmitter::counter(1);
    (
      Transmitter {
        registers: vm::Registers::new(0),
        output: output,
        input: input,
        qsize: qsize.clone(),
        sends: 0,
        ident: 0,
      },
      tx,
      rx,
      qsize.clone(),
    )
  }

  fn new(output: Sender<int>, input: Receiver<int>, qsize: Counter, ident: usize) -> Transmitter {
    Transmitter {
      registers: vm::Registers::new(0),
      output: output,
      input: input,
      qsize: qsize,
      sends: 0,
      ident: ident,
    }
  }

  fn snd(&mut self, arg: &vm::Argument<int>) -> Progression {
    let value = self.registers.get(arg);
    self.output.send(value).expect("Send to ouptut?");

    {
      let _nworking = self.qsize.0.lock().unwrap();
      // eprintln!("SND: {}", *nworking);
      {
        self.qsize.1.notify_all();
      }
    }
    self.sends += 1;
    Ok(1)
  }

  fn set(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self
      .registers
      .get_mut(&args.target)
      .expect("Requires a register!");
    *target = value;
    Ok(1)
  }

  fn add(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self
      .registers
      .get_mut(&args.target)
      .expect("Requires a register!");
    *target += value;
    Ok(1)
  }

  fn mul(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self
      .registers
      .get_mut(&args.target)
      .expect("Requires a register!");
    *target *= value;
    Ok(1)
  }

  fn mod_(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self
      .registers
      .get_mut(&args.target)
      .expect("Requires a register!");
    *target = *target % value;
    Ok(1)
  }

  fn rcv(&mut self, arg: &vm::Argument<int>) -> Progression {
    let mut nworking = self.qsize.0.lock().unwrap();
    *nworking -= 1;

    // eprintln!("{} RCV: {}",self.ident, *nworking);

    // Try recieving at least once.
    let r = match self.input.try_recv() {
      Ok(value) => {
        let target = self.registers.get_mut(&arg).expect("Requires a register!");
        *target = value;
        Ok(1)
      }
      Err(_) => Err("Queue was empty".to_string()),
    };

    match r {
      Ok(value) => {
        *nworking += 1;
        return Ok(value);
      }
      Err(_) => {}
    };

    while *nworking > 0 {
      match self.input.try_recv() {
        Ok(value) => {
          let target = self.registers.get_mut(&arg).expect("Requires a register!");
          *target = value;
          *nworking += 1;
          return Ok(1);
        }
        Err(_) => {
          // eprintln!("{} Deadlock! {}", self.ident, *nworking);
          nworking = self.qsize.1.wait(nworking).unwrap();
        }
      };
    }
    // println!("{} Ending after deadlock: {} {}",self.ident, *nworking,
    // self.sends());
    self.qsize.1.notify_all();
    Err(format!(
      "Deadlock Program:{} No workers remain.",
      self.ident
    ))
  }

  fn jgz(&mut self, args: &Arguments<int>) -> Progression {
    let value = self.registers.get(&args.argument);
    let target = self.registers.get(&args.target);
    if target > 0 {
      Ok(value)
    } else {
      Ok(1)
    }
  }

  /// Close the program, letting others know that this program is done.
  fn close(&mut self) {
    {
      let mut nworking = self.qsize.0.lock().unwrap();
      *nworking -= 1;
    }
    self.qsize.1.notify_all();
  }

  /// Execute the command, and return the offset for the next position
  fn execute(&mut self, cmd: &Command) -> Progression {
    match *cmd {
      Command::Snd(ref arg) => self.snd(arg),
      Command::Set(ref args) => self.set(args),
      Command::Add(ref args) => self.add(args),
      Command::Mul(ref args) => self.mul(args),
      Command::Mod(ref args) => self.mod_(args),
      Command::Rcv(ref arg) => self.rcv(arg),
      Command::Jgz(ref arg) => self.jgz(arg),
    }
  }

  pub fn sends(&self) -> usize {
    self.sends
  }
}

pub struct TransmissionIterator<'a> {
  commands: Vec<Command<'a>>,
  transmitter: Transmitter,
  watchers: Option<(Sender<int>, Receiver<int>)>,
  position: Option<isize>,
}

impl<'a> Iterator for TransmissionIterator<'a> {
  type Item = int;
  fn next(&mut self) -> Option<int> {
    // Program has not expired, iterate:
    if let Some(pos) = self.position {
      // Step through progress until we are
      let mut nextpos = pos;

      // Grab the last sound emitted;
      let mut sound = None;

      loop {
        // Step to the next position
        // eprintln!("{} {:?}", nextpos, self.commands[nextpos as usize]);
        match self.transmitter.execute(&self.commands[nextpos as usize]) {
          Ok(progress) => {
            nextpos += progress;

            // If we are watching, then watch.
            if let Some((ref wtx, ref wrx)) = self.watchers {
              if let Ok(value) = wrx.try_recv() {
                sound = Some(value);
                wtx.send(value).unwrap();
              }
            }

            // We've ended the program,
            // note this fact!
            if !((0 <= nextpos) & (nextpos < self.commands.len() as isize)) {
              self.position = None;
              self.transmitter.close();
              return None;
            }

            // If we are watching, emit sounds.
            if self.watchers.is_some() {
              // Get the most recent sound emitted. Skip rcv.
              if let Command::Rcv(ref arg) = self.commands[nextpos as usize] {
                nextpos += 1;
                if self.transmitter.registers.get(arg) > 0 {
                  self.position = Some(nextpos);
                  return sound;
                }
              }
            } else {
              self.position = Some(nextpos);
              return Some(self.transmitter.sends() as isize);
            }
          }
          Err(_) => {
            // When progress is none
            self.position = None;
            return None;
          }
        }
      }
    } else {
      // Program has expired, signal that we are done.
      self.transmitter.close();
      return None;
    }
  }
}

pub fn run_program<'a>(program: &'a str) -> TransmissionIterator<'a> {
  let commands = program.lines().map(|x| Command::parse(x)).collect();

  let (transmitter, ty, rx, _counter) = Transmitter::single();
  TransmissionIterator {
    commands: commands,
    transmitter: transmitter,
    watchers: Some((ty, rx)),
    position: Some(0),
  }
}

pub fn run_unwatched_program<'a>(
  program: &'a str,
  transmitter: Transmitter,
) -> TransmissionIterator<'a> {
  let commands = program.lines().map(|x| Command::parse(x)).collect();
  TransmissionIterator {
    commands: commands,
    transmitter: transmitter,
    watchers: None,
    position: Some(0),
  }
}

pub fn run_pair(program: &str) -> (usize, usize) {
  let (ta, ra) = channel();
  let (tb, rb) = channel();
  let qsize = Transmitter::counter(2);

  let mut a = Transmitter::new(ta, rb, qsize.clone(), 0);
  a.execute(&Command::parse("set p 0")).unwrap();
  let pa = program.to_string();
  let mut b = Transmitter::new(tb, ra, qsize.clone(), 1);
  b.execute(&Command::parse("set p 1")).unwrap();
  let pb = program.to_string();

  let ta = thread::spawn(move || run_unwatched_program(&pa, a).last().unwrap());
  let tb = thread::spawn(move || run_unwatched_program(&pb, b).last().unwrap());

  let sa = ta.join().unwrap() as usize;
  let sb = tb.join().unwrap() as usize;

  (sa, sb)
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_parse_commands() {
    let cmd = Command::parse("snd a");
    assert_eq!(cmd, Command::Snd(vm::Argument::Register("a")));

    let cmd = Command::parse("set a 10");
    let args = Arguments {
      target: vm::Argument::Register("a"),
      argument: vm::Argument::Value(10),
    };
    assert_eq!(cmd, Command::Set(args));
  }

  #[test]
  fn test_execute_commands() {
    let (mut transmitter, _ty, rx, _counter) = Transmitter::single();

    let p = transmitter.execute(&Command::parse("snd a"));
    assert_eq!(p, Ok(1));
    let p = transmitter.execute(&Command::parse("set a 10"));
    assert_eq!(p, Ok(1));
    let p = transmitter.execute(&Command::parse("add a 1"));
    assert_eq!(p, Ok(1));
    let p = transmitter.execute(&Command::parse("mul b 2"));
    assert_eq!(p, Ok(1));
    let p = transmitter.execute(&Command::parse("mod a 10"));
    assert_eq!(p, Ok(1));
    let p = transmitter.execute(&Command::parse("jgz a 10"));
    assert_eq!(p, Ok(10));
    let p = transmitter.execute(&Command::parse("snd a"));
    assert_eq!(p, Ok(1));
    assert_eq!(rx.recv(), Ok(0));
  }

  #[test]
  fn test_execute_program() {
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

  #[test]
  fn test_pair_program() {
    let program = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
    let (a, b) = run_pair(&program);
    assert_eq!(a, 3);
    assert_eq!(b, 3);
  }

}
