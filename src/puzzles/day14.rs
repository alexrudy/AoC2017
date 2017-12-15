use super::super::knot;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Disk {
  data : Vec<knot::Knot>
}

impl Disk {
  
  pub fn new(key: &str) -> Disk {
    let mut disk = Disk { data: Vec::new() };
    for row in 0..128 {
      let rowkey = format!("{}-{}", key, row);
      let knot = knot::Knot::standard(&rowkey);
      disk.data.push(knot)
    }
    disk
  }
  
  pub fn used(&self) -> usize {
    self.data.iter().map(|row| row.bindigest().chars().filter(|b| { *b == '1' }).count()).sum()
  }
  
  pub fn show(&self) {
    for row in &self.data {
      println!("{}", row.bindigest())
    }
  }
  
  pub fn regions(&self) -> usize {
    
    let mut regions = 0;
    let mut grid : HashSet<(isize, isize)> = HashSet::new();
    for (y, row) in self.data.iter().enumerate() {
      for (x, cell) in row.bindigest().chars().enumerate() {
        if cell == '1' {
          grid.insert((x as isize, y as isize));
        }
      }
    }
    
    let mut queue = VecDeque::new();
    
    loop {
      let cell = {
        match grid.iter().next() {
          Some(c) => *c,
          None => break
        }
      };
      
      grid.remove(&cell);
      regions += 1;
      queue.push_back(cell);
      
      while let Some((x, y)) = queue.pop_front() {
        queue.extend(&grid.take(&(x - 1, y)));
        queue.extend(&grid.take(&(x + 1, y)));
        queue.extend(&grid.take(&(x, y - 1)));
        queue.extend(&grid.take(&(x, y + 1)));
      }
    }
    
    regions
  }
}

#[cfg(test)]
mod tests {
  
  use super::*;
  
  #[test]
  fn example_used() {
    let disk = Disk::new("flqrgnkx");
    assert_eq!(disk.used(), 8108);
  }
  
  #[test]
  fn example_regions() {
    let disk = Disk::new("flqrgnkx");
    assert_eq!(disk.regions(), 1242);
  }
  
}