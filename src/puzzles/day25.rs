use std::collections::{HashSet, HashMap};
use std::str::FromStr;
use std::fmt;

pub struct TuringMachine {
  cursor: isize,

  /// Recorded as the positions of 1
  /// values on the ticker tape.
  tape: HashSet<isize>,
  
  states: HashMap<String, State>,
  
  state: String,
}

pub struct TuringIterator<'a> {
  machine: &'a mut TuringMachine
}

impl<'a> Iterator for TuringIterator<'a> {
  type Item = usize;
  fn next(&mut self) -> Option<usize> {
    self.machine.advance().unwrap();
    Some(self.machine.count())
  }
}

impl FromStr for TuringMachine {
  type Err = ParseTuringError;
  
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let lines = s.lines().collect::<Vec<_>>();
    let istate = second_to_last_character(lines[0]);
    let mut states = HashMap::new();
    for rule in lines[2..].chunks(10) {
      let s : State = rule[1..].join("\n").parse()?;
      states.insert(s.name.clone(), s);
    }
    
    Ok(Self {
      cursor: 0,
      tape: HashSet::new(),
      states: states,
      state: istate.to_string(),
    })
    
  }
  
}

impl TuringMachine {
  
  pub fn count(&self) -> usize {
    self.tape.len()
  }
  
  pub fn iter<'a>(&'a mut self) -> TuringIterator<'a> {
    TuringIterator {
      machine: self
    }
  }
  
  fn advance(&mut self) -> Result<(), MachineError> {
    let state = self.states.get(&self.state).ok_or(MachineError::InvalidState)?;
    let rule = if self.tape.contains(&self.cursor) {&state.one} else {&state.zero};
    match rule.value {
      Value::Zero => {self.tape.remove(&self.cursor);},
      Value::One => {self.tape.insert(self.cursor);},
    }
    match rule.direction {
      Direction::Left => {self.cursor -= 1;},
      Direction::Right => {self.cursor += 1;},
    }
    self.state = rule.state.clone();
    Ok(())
  }
  
  /// Inflate the turing machine into a full tape,
  /// of 1s and zeors. Note that this looses the center
  /// position, as vectors already index starting at zero.
  fn inflate(&self) -> Vec<usize> {
    if self.tape.is_empty() {
      return Vec::new();
    }

    // We must get the largest and smallest value.
    let ends: (isize, isize) = self.tape.iter().fold((0, 0), |mut state, &i| {
      if i < state.0 {
        state.0 = i
      };
      if i > state.1 {
        state.1 = i
      };
      state
    });

    let tape_size = (ends.1 - ends.0 + 1) as usize;

    let mut tape = Vec::with_capacity(tape_size);
    tape.resize(tape_size, 0);
    for i in self.tape.iter() {
      tape[(i - ends.0) as usize] = 1;
    }
    tape
  }
}

impl fmt::Display for TuringMachine {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let tape = self.inflate();
    write!(f, "T:{}{:?}@{}", self.state, tape, self.cursor)?;
    Ok(())
  }
}

use std::num::ParseIntError;

#[derive(Debug, Fail)]
pub enum MachineError {
  #[fail(display = "Invalid State")]
  InvalidState,
}

#[derive(Debug, Fail)]
pub enum ParseTuringError {
  #[fail(display = "Failed to parse integer")] ParseValue(#[cause] ParseIntError),

  #[fail(display = "Failed to parse integer")] InvalidInteger(usize),

  #[fail(display = "Failed to parse direction")] ParseDirection,

  #[fail(display = "Missing a line.")] MissingLine,
}

impl From<ParseIntError> for ParseTuringError {
  fn from(e: ParseIntError) -> ParseTuringError {
    ParseTuringError::ParseValue(e)
  }
}

#[derive(Debug, PartialEq)]
enum Value {
  Zero,
  One,
}

impl FromStr for Value {
  type Err = ParseTuringError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().parse::<usize>()? {
      0 => Ok(Value::Zero),
      1 => Ok(Value::One),
      i @ _ => Err(ParseTuringError::InvalidInteger(i)),
    }
  }
}

#[derive(Debug, PartialEq)]
enum Direction {
  Left,
  Right,
}

impl FromStr for Direction {
  type Err = ParseTuringError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim() {
      "right" => Ok(Direction::Right),
      "left" => Ok(Direction::Left),
      _ => Err(ParseTuringError::ParseDirection),
    }
  }
}

#[derive(Debug, PartialEq)]
struct Rule {
  value: Value,
  state: String,
  direction: Direction,
}

impl FromStr for Rule {
  type Err = ParseTuringError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut lines = s.lines();
    let nv: Value =
      second_to_last_character(lines.next().ok_or(ParseTuringError::MissingLine)?).parse()?;
    let direction: Direction = lines
      .next()
      .ok_or(ParseTuringError::MissingLine)?
      .split_whitespace()
      .last()
      .ok_or(ParseTuringError::MissingLine)?
      .trim_matches('.')
      .parse()?;
    let es =
      second_to_last_character(lines.next().ok_or(ParseTuringError::MissingLine)?).to_string();

    Ok(Self {
      value: nv,
      state: es,
      direction: direction,
    })
  }
}

#[derive(Debug, PartialEq)]
struct State {
  name: String,
  zero: Rule,
  one: Rule,
}

fn second_to_last_character<'a>(s: &'a str) -> &'a str {
  let st = s.trim();
  let si = st.len() - 2;
  &st[si..(st.len() - 1)]
}

impl FromStr for State {
  type Err = ParseTuringError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut lines = s.lines();
    let state =
      second_to_last_character(lines.next().ok_or(ParseTuringError::MissingLine)?).to_string();
    let cv: Value =
      second_to_last_character(lines.next().ok_or(ParseTuringError::MissingLine)?).parse()?;
    let one: Rule;
    let zero: Rule;

    let lines = lines.collect::<Vec<_>>();
    if cv == Value::One {
      one = lines[..3].join("\n").parse()?;
      zero = lines[4..].join("\n").parse()?;
    } else {
      zero = lines[..3].join("\n").parse()?;
      one = lines[4..].join("\n").parse()?;
    }

    Ok(Self {
      name: state,
      zero: zero,
      one: one,
    })
  }
}

#[cfg(test)]
mod test {

  static RULES: &str = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

  use super::*;

  #[test]
  fn parse_rule() {
    let lines = RULES.lines().collect::<Vec<_>>();
    let state: State = lines[3..12].join("\n").parse().unwrap();
    assert_eq!(&state.name, "A");
    assert_eq!(
      state.zero,
      Rule {
        state: "B".to_string(),
        direction: Direction::Right,
        value: Value::One,
      }
    );
    assert_eq!(
      state.one,
      Rule {
        state: "B".to_string(),
        direction: Direction::Left,
        value: Value::Zero,
      }
    );
  }
  
  #[test]
  fn run_machine() {
    let mut tm : TuringMachine = RULES.parse().unwrap();
    assert_eq!(tm.iter().take(6).last(), Some(3));
  }
}
