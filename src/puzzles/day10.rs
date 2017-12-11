fn knot_once(list: &mut [usize], length: usize, skip_length: usize) -> usize {
  {
    let (segment, _) = list.split_at_mut(length);
    segment.reverse();
  }
  let offset = (length + skip_length) % list.len();
  list.rotate(offset);
  
  return length + skip_length;
}

pub fn knot_hash(inputs: &[usize], length: usize) -> usize {
  let mut skip_length = 0;
  let mut offset = 0;
  let result;
  let mut data : Vec<usize> = (0..length).collect();
  
  for input in inputs {
    offset += knot_once(&mut data, *input, skip_length);
    skip_length += 1;
  }
  
  let offrotate = (length as isize - (offset as isize % length as isize)).abs() as usize;
  data.rotate(offrotate);
  
  {
    let mut diter = data.iter().take(2);
    let a = diter.next().unwrap();
    let b = diter.next().unwrap();
    
    result = a * b;
  }
  result
}

#[cfg(test)]
mod test {
  
  use super::*;
  
  #[test]
  fn do_knot_hash() {
    
    let inputs = vec![3, 4, 1, 5];
    assert_eq!(knot_hash(&inputs, 5), 12);
    
  }
  
  #[test]
  fn do_one_knot() {
    
    let mut start = 0;
    let mut data = vec![0, 1, 2, 3, 4];
    start += knot_once(&mut data, 3, 0);
    
    let offset = (data.len() - start) % data.len();
    data.rotate(offset);
    
    assert_eq!(data, vec![2, 1, 0, 3, 4]);
  }
}