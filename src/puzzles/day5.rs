// Make a single jump.
pub fn jump_instruction(instructions: &mut Vec<isize>, position: isize) -> Result<isize, String> {
  let jump = instructions[position as usize];
  instructions[position as usize] += 1;
  if jump < -position || position + jump > (instructions.len() as isize) {
    return Err("Out of bounds.".to_string());
  }
  return Ok(position + jump);
}

pub fn jump_fancy(instructions: &mut Vec<isize>, position: isize) -> Result<isize, String> {
  let jump = instructions[position as usize];
  if jump >= 3 {
    instructions[position as usize] -= 1;
  } else {
    instructions[position as usize] += 1;
  }
  if jump < -position || position + jump > (instructions.len() as isize) {
    return Err("Out of bounds.".to_string());
  }
  return Ok(position + jump);
}

pub fn run_instructions(
  instructions: &mut Vec<isize>,
  position: isize,
  jumper: fn(&mut Vec<isize>, isize) -> Result<isize, String>,
) -> Result<isize, String> {
  let mut nsteps = 0;
  let mut position = position;
  while (position as usize) < instructions.len() {
    position = jumper(instructions, position)?;
    nsteps += 1;
  }
  return Ok(nsteps);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn jump_maze() {
    let mut instructions = vec![0, 3, 0, 1, -3];
    assert_eq!(
      run_instructions(&mut instructions, 0, jump_instruction).unwrap(),
      5
    );
  }

  #[test]
  fn jump_fancy_maze() {
    let mut instructions = vec![0, 3, 0, 1, -3];
    assert_eq!(
      run_instructions(&mut instructions, 0, jump_fancy).unwrap(),
      10
    );
  }

  #[test]
  fn jump_once() {
    let mut instructions = vec![0, 3, 0, 1, -3];
    assert_eq!(jump_instruction(&mut instructions, 0).unwrap(), 0);
    assert_eq!(instructions, vec![1, 3, 0, 1, -3]);
  }

}
