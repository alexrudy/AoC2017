use std::collections::HashSet;
use std::iter::FromIterator;

pub fn check_passphrase(passphrase: &str) -> bool {
  let mut words = HashSet::new();
  
  for word in passphrase.split(char::is_whitespace) {
    let word = word.trim();
    if words.contains(&word) {
      return false;
    } else {
      words.insert(word);
    }
  }
  true
}

pub fn check_passphrase_anagrams(passphrase: &str) -> bool {
  let mut words = HashSet::new();
  
  for word in passphrase.split(char::is_whitespace) {
    let mut chars: Vec<char> = word.trim().chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    let word = String::from_iter(chars);
    if words.contains(&word) {
      return false;
    } else {
      words.insert(word);
    }
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn passphrases_test() {
    assert!(check_passphrase("aa bb cc dd ee"));
    assert!(!check_passphrase("aa bb cc dd aa"));
    assert!(check_passphrase("aa bb cc dd aaa"));
  }
  
  #[test]
  fn anagram_test() {
    assert!(check_passphrase_anagrams("abcde fghij"));
    assert!(!check_passphrase_anagrams("abcde xyz ecdab"));
    assert!(check_passphrase_anagrams("a ab abc abd abf abj"));
    assert!(check_passphrase_anagrams("a ab abc abd abf abj"));
    assert!(check_passphrase_anagrams("iiii oiii ooii oooi oooo"));
    assert!(!check_passphrase_anagrams("oiii ioii iioi iiio"));
    
    
  }
}