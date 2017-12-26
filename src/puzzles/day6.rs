use std::collections::HashMap;


fn reallocate_memory(memory_bank: &mut Vec<usize>) {
  let largest = memory_bank
    .iter()
    .enumerate()
    .rev()
    .max_by_key(|x| x.1)
    .unwrap()
    .0;
  let values = memory_bank[largest];
  let vsize = memory_bank.len();
  memory_bank[largest] = 0;
  for i in 0..values {
    let index = (largest + i + 1) % vsize;
    memory_bank[index] += 1;
  }
}

pub fn reallocate_many(mut memory_bank: Vec<usize>) -> (u32, u32) {
  let mut seen = HashMap::new();
  let mut nsteps: u32 = 0;
  while !seen.contains_key(&memory_bank) {
    seen.insert(memory_bank.clone(), nsteps);
    reallocate_memory(&mut memory_bank);
    nsteps += 1;
  }
  (nsteps, nsteps - seen.get(&memory_bank).unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn memory_move() {
    let mut memory = vec![0, 2, 7, 0];
    reallocate_memory(&mut memory);
    assert_eq!(memory, vec![2, 4, 1, 2]);
  }

  #[test]
  fn memory_move_many() {
    let memory = vec![0, 2, 7, 0];
    assert_eq!(reallocate_many(memory), (5, 4));
  }

}
