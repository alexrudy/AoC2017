use super::super::hexagons;

pub fn distance(path: &str) -> Result<i32, String> {
  let moves : Vec<&str> = path.split(',').collect();
  Ok(trace(&moves)?.last().ok_or("No elements!")?.distance(hexagons::HexPoint::origin()))
}

pub fn maxdistance(path: &str) -> Result<i32, String> {
  let moves : Vec<&str> = path.split(',').collect();
  trace(&moves)?.iter().map(|x| x.distance(hexagons::HexPoint::origin())).max().ok_or("No elements!".to_string())
}

fn trace(path: &[&str]) -> Result<Vec<hexagons::HexPoint>, String> {
  let start = hexagons::HexPoint::origin();
  Ok(path.iter().scan(start, |pos, x| {
    *pos = pos.hexmove(x).unwrap();
    Some(*pos)
  }).collect())
}

#[cfg(test)]
mod test {
  
  use super::*;
  
  #[test]
  fn hexdistance() {
    assert_eq!(distance("ne,ne,ne").unwrap(), 3);
    assert_eq!(distance("ne,ne,sw,sw").unwrap(), 0);
    assert_eq!(distance("ne,ne,s,s").unwrap(), 2);
    assert_eq!(distance("se,sw,se,sw,sw").unwrap(), 3);
  }
  
  #[test]
  fn maxhexdistance() {
    assert_eq!(maxdistance("ne,ne,ne").unwrap(), 3);
    assert_eq!(maxdistance("ne,ne,sw,sw").unwrap(), 2);
    assert_eq!(maxdistance("ne,ne,s,s").unwrap(), 2);
    assert_eq!(maxdistance("se,sw,se,sw,sw").unwrap(), 3);
  }
  
}