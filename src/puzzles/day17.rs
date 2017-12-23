
pub fn spinlock(length: usize, skip: usize) -> Vec<usize> {
  
  let mut buffer = Vec::with_capacity(length);
  let mut position = 0;
  
  for i in 1..length {
    position = (position + skip + 1) % i;
    buffer.insert(position, i);
  }
  
  buffer
}

pub fn spinlock_afterzero(length: usize, skip: usize) -> usize {
  let mut vaz = 0;
  let mut position = 0;
  for i in 1..length {
    position = (position + skip) % i;
    if position == 0 {
      vaz = i;
    }
    position += 1;
  }
  vaz
}

pub fn getnextitem(lock: &[usize], target: usize) -> usize {
  let n = (lock.iter().enumerate().find(|&(_i, x)| { *x == target }).unwrap().0 + 1) % lock.len();
  lock[n]
}

#[cfg(test)]
mod test {
  
  use super::*;
  
  #[test]
  fn small_lock() {
    let locked = spinlock(2018, 3);
    assert_eq!(getnextitem(&locked, 2017), 638);
  }
}