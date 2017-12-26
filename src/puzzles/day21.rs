use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Fail)]
pub enum PatchError {
  
  #[fail(display = "invalid patch shape ({},{})", width, height)]
  InvalidPatchShape {
    width: usize,
    height: usize,
  },
  
  #[fail(display = "unspliattable patch with size {}", size)]
  UnsplittablePatch {
    size: usize,
  },
  
  #[fail(display = "invalid rule \"{}\"", rule)]
  RuleError {
    rule: String,
  }
}

type Cells = Vec<Vec<bool>>;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Patch {
  cells: Cells,
}

impl FromStr for Patch {
  type Err = PatchError;
  
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut cells : Cells = Vec::new();
    for line in s.split("/") {
      cells.push(line.trim().chars().map(|x| { x == '#' }).collect());
    }
    let patch = Patch { cells: cells };
    if let Some(e) = patch.consistent() {
      return Err(e)
    }
    Ok(patch)
  }
}

impl fmt::Debug for Patch {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "Patch {{ ")?;
    write!(f, "\"{}\"", self.cells.iter().map(|r| {
      r.iter().map(|c| if *c {"#"} else {"."}).collect::<String>()
      }).collect::<Vec<String>>().join("/"))?;
    write!(f, " }}")?;
    Ok(())
  }
}

impl fmt::Display for Patch {
  
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    for row in self.cells.iter() {
      for col in row.iter() {
        if *col {
          write!(f, "#")?;
        } else {
          write!(f, ".")?;
        }
      }
      writeln!(f, "")?;
    }
    Ok(())
  }
}

impl Patch {
  
  fn start() -> Self {
    ".#./..#/###".parse().unwrap()
  }
  
  fn new(size: usize) -> Self {
    let mut row = Vec::with_capacity(size);
    row.resize(size, false);
    let mut cells = Vec::with_capacity(size);
    cells.resize(size, row);
    Self { cells : cells }
  }
  
  fn from_coordinates(size: usize, values: &[(usize, usize, bool)]) -> Self {
    let mut patch = Self::new(size);
    for &(r, c, v) in values {
      patch.set((r, c), v);
    }
    patch
  }
  
  // Join and split patterns
  fn mutate(patches: &[&Patch]) -> Result<Vec<Self>, PatchError> {
    let mut mutations = Vec::new();
    
    let psize = patches[0].size();
    let npatches = (patches.len() as f64).sqrt() as usize;
    let ncells = npatches * psize;
    let msize;
    if ncells % 2 == 0 {
      msize = 2;
    } else if ncells % 3 == 0 {
      msize = 3;
    } else {
      return Err(PatchError::UnsplittablePatch{ size: npatches });
    }
    
    
    let m = ncells / msize;
    let mut shuffled = HashMap::new();
    
    for (i, patch) in patches.iter().enumerate() {
      
      let pc = i % npatches;
      let pr = i / npatches;
      
      for (cc, col) in patch.cells.iter().enumerate() {
        for (cr, value) in col.iter().enumerate() {
          let oc = pc * psize + cc;
          let or = pr * psize + cr;
          
          let dc = oc / msize;
          let dr = or / msize;
          
          {
            let vref = shuffled.entry((dc, dr)).or_insert(Vec::new());
            (*vref).push((oc % msize, or % msize, *value));
          }
        }
      }
    }
    
    for i in 0..shuffled.len() {
      
      //TODO: This looks really backwards to me, I'm not sure what is going on here.
      let dr = i % m;
      let dc = i / m;
      let coords = shuffled.get(&(dc, dr)).unwrap();
      mutations.push(Patch::from_coordinates(msize, coords));
    }
    
    Ok(mutations)
  }
  
  // Check for internal consistency
  fn consistent(&self) -> Option<PatchError> {
    if !self.cells.iter().all(|x| x.len() == self.cells.len()) {
      return Some(PatchError::InvalidPatchShape{ width:self.cells.len(), height:self.cells[0].len()})
    }
    None
  }
  
  fn on(&self) -> usize {
    self.cells.iter().map(|r| r.iter().filter(|&x| {*x}).count()).sum()
  }
  
  fn size(&self) -> usize {
    self.cells.len()
  }
  
  fn set(&mut self, index: (usize, usize), value: bool) {
    let (x, y) = index;
    self.cells[x][y] = value;
  }
  
  fn get(&mut self, index: (usize, usize)) -> bool {
    let (x, y) = index;
    self.cells[x][y]
  }
  
  fn transpose(&self) -> Self {
    let mut new_patch = Self::new(self.size());
    for (x, r) in self.cells.iter().enumerate() {
      for (y, c) in r.iter().enumerate() {
        new_patch.set((y, x), *c);
      }
    }
    new_patch
  }
  
  fn flip(&self) -> Self {
    Self{ cells:self.cells.iter().map(|x| x.iter().rev().cloned().collect()).collect() }
  }
  
  fn rotate(&self) -> Self {
    self.flip().transpose()
  }
  
  pub fn variants(&self) -> Vec<Self> {
    let mut variants = Vec::with_capacity(8);
    variants.push(self.clone());
    variants.push(self.flip());
    for i in 0..6 {
      let variant = variants[i].rotate();
      variants.push(variant);
    }
    variants
  }
}

pub struct PatternSet {
  patterns : HashMap<Patch, Patch>
}

impl PatternSet {
  
  pub fn new() -> Self {
    Self { patterns: HashMap::new() }
  }
  
  pub fn insert(&mut self, s: &str) -> Result<(), PatchError> {
    
    let mut parts = s.split("=>");
    
    let left : Patch = parts.next().ok_or_else(|| PatchError::RuleError{ rule: s.to_string() })?.parse()?;
    let right : Patch = parts.next().ok_or_else(|| PatchError::RuleError{ rule: s.to_string() })?.parse()?;
    
    for variant in left.variants() {
      self.patterns.insert(variant, right.clone());
    }
    
    Ok(())
  }
  
  pub fn get(&self, pattern: &Patch) -> Option<&Patch> {
    self.patterns.get(pattern)
  }
}

#[cfg(test)]
mod test {
  
  use super::*;
  
  use std::collections::HashSet;
  
  #[test]
  fn parse_patch() {
    let patch : Patch = "../.#".parse().unwrap();
    assert_eq!(patch.size(), 2);
  }
  
  #[test]
  fn rotate_patch() {
    let patch : Patch = "../.#".parse().unwrap();
    let p90 = patch.rotate();
    assert_eq!(p90, ".#/..".parse::<Patch>().unwrap());
  }
  
  #[test]
  fn vary_patch() {
    let patch : Patch = "../.#".parse().unwrap();
    let varpatches = patch.variants();
    assert_eq!(varpatches.len(), 8);
    let unique_variants : HashSet<Patch> = varpatches.iter().cloned().collect();
    assert_eq!(unique_variants.len(), 4);
  }
  
  #[test]
  fn mutate_patches() {
    let patch : Patch = ".../.#./...".parse().unwrap();
    let mut patches = Vec::with_capacity(4);
    patches.resize(4, &patch);
    
    let new_patches = Patch::mutate(&patches).unwrap();
    println!("{:?}", new_patches);
    assert_eq!(new_patches.len(), 9);
  }
  
  #[test]
  fn parse_rules() {
    let mut rules = PatternSet::new();
    rules.insert("../.# => ##./#../...").unwrap();
    rules.insert(".#./..#/### => #..#/..../..../#..#").unwrap();
  }
  
  #[test]
  fn example() {
    let mut rules = PatternSet::new();
    rules.insert("../.# => ##./#../...").unwrap();
    rules.insert(".#./..#/### => #..#/..../..../#..#").unwrap();
    
    let first_patch = Patch::start();
    let mut patch = &first_patch;
    
    patch = rules.get(patch).unwrap();
    assert_eq!(patch, &"#..#/..../..../#..#".parse::<Patch>().unwrap());
    
    let mut mutated = Patch::mutate(&vec![patch]).unwrap();
    
    {
      let expected = vec![
      "#./..".parse::<Patch>().unwrap(),
      ".#/..".parse::<Patch>().unwrap(),
      "../#.".parse::<Patch>().unwrap(),
      "../.#".parse::<Patch>().unwrap(),    
      ];
      assert_eq!(mutated, expected);
    }
    
    mutated = mutated.iter().map(|p| rules.get(p).unwrap().clone()).collect();
    {
      let expected = vec![
      "##./#../...".parse::<Patch>().unwrap(),
      "##./#../...".parse::<Patch>().unwrap(),
      "##./#../...".parse::<Patch>().unwrap(),
      "##./#../...".parse::<Patch>().unwrap(),    
      ];
      assert_eq!(mutated, expected);
    }
    assert_eq!(mutated.iter().map(|x| x.on()).sum::<usize>(), 12);
  }
  
}
