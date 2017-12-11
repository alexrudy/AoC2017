fn knot_once(list: &mut [u8], length: usize, skip_length: usize) -> usize {
  let split = length % list.len();
  {
    let (segment, _) = list.split_at_mut(split);
    segment.reverse();
  }
  let offset = (length + skip_length) % list.len();
  list.rotate(offset);
  
  return length + skip_length;
}

fn knot_extend(data: &mut Vec<u8>) {
  data.extend(vec![17, 31, 73, 47, 23]);
}

// Produce a digest of knot strings.
fn knot_digest(data: &[u8]) -> String {
  let chunkdata : String = data.chunks(16).map(|chunk| chunk.iter().fold(0, | acc, &x | acc ^ x))
                               .map(|x| format!("{:02x}", x)).collect();
  chunkdata
}

pub fn knot_simple_digest(data: &[u8]) -> u32 {
  let mut diter = data.iter().take(2);
  let a = *diter.next().unwrap() as u32;
  let b = *diter.next().unwrap() as u32;
  
  a * b
}

pub fn knot(inputs: &str) -> String {
  
  let mut binputs : Vec<u8> = inputs.bytes().collect();
  knot_extend(&mut binputs);
  let uinputs : Vec<usize> = binputs.iter().map(|&x| x as usize).collect();
  let hdata = knot_hash(&uinputs, 256, 64);
  knot_digest(&hdata)
}

pub fn knot_hash(inputs: &[usize], length: usize, rounds: usize) -> Vec<u8> {
  let mut skip_length = 0;
  let mut offset = 0;
  let mut data : Vec<u8> = (0..length).map(|x| {x as u8}).collect();
  
  for _i in 0..rounds {
    for input in inputs {
      offset += knot_once(&mut data, *input, skip_length);
      skip_length += 1;
    }
  }
  
  let offrotate = (length as isize - (offset as isize % length as isize)).abs() as usize;
  data.rotate(offrotate);
  
  data
}

#[cfg(test)]
mod test {
  
  use super::*;
  
  #[test]
  fn try_hashes () {
    assert_eq!(&knot("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(&knot("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(&knot("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    assert_eq!(&knot(""), "a2582a3a0e66e6e86e3812dcb672a272");
  }
  
  
  #[test]
  fn do_part_one () {
    
    let raw_input = "189,1,111,246,254,2,0,120,215,93,255,50,84,15,94,62";
    let input : Vec<usize> = raw_input.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    
    let khash = knot_hash(&input, 256, 1);
    assert_eq!(knot_simple_digest(&khash), 38415);
  }
  
  #[test]
  fn do_knot_hash() {
    
    let inputs = vec![3, 4, 1, 5];
    let khash = knot_hash(&inputs, 5, 1);
    assert_eq!(knot_simple_digest(&khash), 12);
    
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