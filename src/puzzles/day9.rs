
pub fn procress_stream(text: &str) -> (u32, u32) {
  
  let mut score = 0;
  let mut garbage = false;
  let mut ngarbage = 0;
  let mut group = 0;
  let mut chars = text.chars();
  
  while let Some(ch) = chars.next() {
    match ch {
      '!' => { chars.next(); },
      '>' => { garbage = false; },
      _ if garbage => { ngarbage += 1; },
      '<' => { garbage = true; },
      '{' => { group += 1; },
      '}' => { score += group; group -= 1; },
      _ => {},
    }    
  }
  return (score, ngarbage)
}

#[cfg(test)]
mod test {
  
  use super::*;
  
  #[test]
  fn try_score_stream() {
    assert_eq!(procress_stream("{}"), (1, 0));
    assert_eq!(procress_stream("{{{}}}"), (6, 0));
    assert_eq!(procress_stream("{{},{}}"), (5, 0));
    assert_eq!(procress_stream("{{{},{},{{}}}}"), (16, 0));
    assert_eq!(procress_stream("{<a>,<a>,<a>,<a>}"), (1, 4));
    assert_eq!(procress_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
  }
  
  #[test]
  fn try_garbage_stream () {
    assert_eq!(procress_stream("<>").1, 0);
    assert_eq!(procress_stream("<random characters>").1, 17);
    assert_eq!(procress_stream("<<<<>").1, 3);
    assert_eq!(procress_stream("<{!>}>").1, 2);
    assert_eq!(procress_stream("<!!>").1, 0);
    assert_eq!(procress_stream("<!!!>>").1, 0);
    assert_eq!(procress_stream("<{o\"i!a,<{i<a>").1, 10);
  }
  
}