//! Day 21: Fractal Art
//!
//! Tools for working with the fractal art pieces.

use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;

/// Module level error type used to handle the various
/// error cases where rule construction might fail.
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

  #[fail(display = "invalid rule \"{}\"", rule)] RuleError {
    rule: String,
  },

  #[fail(display = "missing rule for \"{}\"", patch)] MissingRule {
    patch: Patch,
  },

  #[fail(display = "duplicate rule for \"{}\"", patch)] DuplicateRule {
    patch: Patch,
  },

  #[fail(display = "{}", _0)] Io(#[cause] io::Error),
}

type Result<T> = ::std::result::Result<T, PatchError>;

/// The type used internally to hold cells,
/// aliased here for some conveninece.
type Cells = Vec<Vec<bool>>;

/// A single patch of artwork, consisting of `#` and `.` in a square grid.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Patch {
  cells: Cells,
}

impl FromStr for Patch {
  type Err = PatchError;

  fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
    let mut cells: Cells = Vec::new();
    for line in s.split("/") {
      cells.push(line.trim().chars().map(|x| x == '#').collect());
    }
    let patch = Patch { cells: cells };
    if let Some(e) = patch.consistent() {
      return Err(e);
    }
    Ok(patch)
  }
}

impl fmt::Debug for Patch {
  fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
    write!(f, "Patch {{ ")?;
    write!(
      f,
      "\"{}\"",
      self
        .cells
        .iter()
        .map(|r| r.iter()
          .map(|c| if *c { "#" } else { "." })
          .collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
    )?;
    write!(f, " }}")?;
    Ok(())
  }
}

impl fmt::Display for Patch {
  fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
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

impl Default for Patch {
  /// Get the default starting patch
  /// for the day 21 puzzle:
  ///
  /// ```
  /// use aoc2017::puzzles::day21;
  ///
  /// let patch = day21::Patch::default();
  /// assert_eq!(format!("{:?}", patch), "Patch { \".#./..#/###\" }");
  /// ```
  fn default() -> Self {
    ".#./..#/###".parse().unwrap()
  }
}

impl Patch {
  /// Create a new empty patch with a given size.
  /// For example, if `size = 3`, The patch will have the layout
  /// `.../.../...`
  pub fn new(size: usize) -> Self {
    let mut row = Vec::with_capacity(size);
    row.resize(size, false);
    let mut cells = Vec::with_capacity(size);
    cells.resize(size, row);
    Self { cells: cells }
  }
  
  #[allow(dead_code)]
  fn from_coordinates(size: usize, values: &[(usize, usize, bool)]) -> Self {
    let mut patch = Self::new(size);
    let mut seen = HashSet::new();
    for &(x, y, v) in values {
      if seen.insert((x, y)) {
        patch.set((x, y), v);
      } else {
        panic!(format!("Duplicate insert! {:?}", values));
      }
    }
    patch
  }
  
  #[allow(dead_code)]
  fn combine(patches: &[&Patch]) -> Self {
    let subpatch_side = patches[0].size();
    let num_subpatch_side = (patches.len() as f64).sqrt() as usize;
    let patch_side = subpatch_side * num_subpatch_side;

    let mut patch = Self::new(patch_side);
    for (idx, subpatch) in patches.iter().enumerate() {
      let gx = idx % num_subpatch_side;
      let gy = idx / num_subpatch_side;
      for x in 0..subpatch_side {
        for y in 0..subpatch_side {
          patch.set(
            (y + gy * subpatch_side, x + gx * subpatch_side),
            subpatch.get((y, x)),
          );
        }
      }
    }
    patch
  }

  #[allow(dead_code)]
  fn split(&self) -> Result<Vec<Self>> {
    let patch_size = self.size();
    let subpatch_size = match patch_size {
      x if x % 2 == 0 => 2,
      x if x % 3 == 0 => 3,
      _ => {
        return Err(PatchError::UnsplittablePatch { size: patch_size });
      }
    };

    let n_subpatches = patch_size / subpatch_size;
    Ok(
      (0..(n_subpatches * n_subpatches))
        .map(|subpatch_idx| {
          let mut patch = Self::new(subpatch_size);
          let sy = subpatch_idx % n_subpatches;
          let sx = subpatch_idx / n_subpatches;
          for x in 0..subpatch_size {
            for y in 0..subpatch_size {
              patch.set(
                (x, y),
                self.get((x + sx * subpatch_size, y + sy * subpatch_size)),
              );
            }
          }

          patch
        })
        .collect(),
    )
  }

  // Join and split patterns
  #[allow(dead_code)]
  fn mutate(patches: &[&Patch]) -> Result<Vec<Self>> {

    let inpatch_size = patches[0].size();
    let num_inpatch_side = (patches.len() as f64).sqrt() as usize;
    let patch_size = num_inpatch_side * inpatch_size;
    let subpatch_size = match patch_size {
      x if x % 2 == 0 => 2,
      x if x % 3 == 0 => 3,
      _ => {
        return Err(PatchError::UnsplittablePatch { size: patch_size });
      }
    };

    let num_subpatch_side = patch_size / subpatch_size;
    
    
    let mut mutations = Vec::with_capacity(num_subpatch_side * num_subpatch_side);
    let subpatch = Patch::new(subpatch_size);
    mutations.resize(num_subpatch_side * num_subpatch_side, subpatch);
    
    for (i, subpatch) in patches.iter().enumerate() {
      // Coordinates of this patch.
      let inpatch_sx = i % num_inpatch_side;
      let inpatch_sy = i / num_inpatch_side;

      for (inpatch_y, col) in subpatch.cells.iter().enumerate() {
        for (inpatch_x, value) in col.iter().enumerate() {
          // Overall coordinates in the mater patch.
          let patch_x = inpatch_sx * inpatch_size + inpatch_x;
          let patch_y = inpatch_sy * inpatch_size + inpatch_y;

          // Coordinates of the subpatch.
          let sx = patch_x / subpatch_size;
          let sy = patch_y / subpatch_size;

          let subpatch_x = patch_x % subpatch_size;
          let subpatch_y = patch_y % subpatch_size;

          {
            let outpatch_i = sx * num_subpatch_side + sy;
            let mut outpatch = &mut mutations[outpatch_i];
            outpatch.set((subpatch_x, subpatch_y), *value);
          }
        }
      }
    }
    Ok(mutations)
  }

  // Check for internal consistency
  fn consistent(&self) -> Option<PatchError> {
    if !self.cells.iter().all(|x| x.len() == self.cells.len()) {
      return Some(PatchError::InvalidPatchShape {
        width: self.cells.len(),
        height: self.cells[0].len(),
      });
    }
    None
  }

  fn on(&self) -> usize {
    self
      .cells
      .iter()
      .map(|r| r.iter().filter(|&x| *x).count())
      .sum()
  }

  fn size(&self) -> usize {
    self.cells.len()
  }

  fn set(&mut self, index: (usize, usize), value: bool) {
    let (x, y) = index;
    self.cells[x][y] = value;
  }

  fn get(&self, index: (usize, usize)) -> bool {
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
    Self {
      cells: self
        .cells
        .iter()
        .map(|x| x.iter().rev().cloned().collect())
        .collect(),
    }
  }

  fn rotate(&self) -> Self {
    self.flip().transpose()
  }

  /// Return a vector over the variants of this
  /// patch. Variants are rotations and flips of
  /// the patch which should be treated identically
  /// in rule input.
  pub fn variants(&self) -> Vec<Self> {
    let mut seen = HashSet::new();
    let mut variants = Vec::with_capacity(8);
    if seen.insert(self.clone()) {
      variants.push(self.clone());
    }
    if seen.insert(self.flip()) {
      variants.push(self.flip());
    }
    for i in 0..(3 * variants.len()) {
      if i >= variants.len() {
        break;
      }
      let variant = variants[i].rotate();
      if seen.insert(variant.clone()) {
        variants.push(variant);
      }
    }
    variants
  }
}

/// Count the number of `#` pixels in a slice of artworks.
pub fn on(patterns: &[&Patch]) -> usize {
  patterns.iter().map(|x| x.on()).sum::<usize>()
}

/// Return the result of the pattern after n iterations.
pub fn after_n<'a>(patterns: &'a PatternSet, n: usize) -> Result<Vec<&'a Patch>> {
  Ok(patterns.iter()?.take(n).last().unwrap()?)
}

/// A collection of rules for transforming one patch
/// into another patch. Patch rules are invariant over the input
/// for rotation and flips. When a new
pub struct PatternSet {
  patterns: HashMap<Patch, Patch>,
}

impl PatternSet {
  /// Create a new, empty pattern set.
  pub fn new() -> Self {
    Self {
      patterns: HashMap::new(),
    }
  }

  fn insert_patch(&mut self, left: &Patch, right: &Patch) -> Result<()> {
    for variant in left.variants() {
      match self.patterns.insert(variant, right.clone()) {
        Some(patch) => {
          return Err(PatchError::DuplicateRule { patch });
        }
        None => {}
      };
    }
    Ok(())
  }

  /// Parse and insert a new rule into this rule map.
  /// This method automatically handles the variants of the input rule,
  /// rotating and flipping the left side (rule trigger) as appropriate.
  /// Input rules should have the form `../.# => ##./#../...`.
  pub fn insert(&mut self, s: &str) -> Result<()> {
    let mut parts = s.split("=>");

    let left: Patch = parts
      .next()
      .ok_or_else(|| PatchError::RuleError {
        rule: s.to_string(),
      })?
      .parse()?;
    let right: Patch = parts
      .next()
      .ok_or_else(|| PatchError::RuleError {
        rule: s.to_string(),
      })?
      .parse()?;

    self.insert_patch(&left, &right)
  }

  /// Apply a rule to get a new pattern.
  pub fn get(&self, pattern: &Patch) -> Option<&Patch> {
    self.patterns.get(pattern)
  }
  
  /// Get a reference to the pattern matching the input pattern.
  /// This allows you to have a reference to the pattern owned
  /// by this pattern set object, which should help with the lifetime. 
  pub fn getkey(&self, pattern: &Patch) -> Option<&Patch> {
    self.patterns.keys().find(|x| x == &pattern)
  }

  /// Apply rules to all of the passed patterns, returning the new patterns.
  pub fn transform<'a>(&'a self, patterns: &[&Patch]) -> Result<Vec<&'a Patch>> {
    patterns
      .iter()
      .map(|p| {
        self.get(p).ok_or_else(|| PatchError::MissingRule {
          patch: (*p).clone(),
        })
      })
      .collect()
  }

  /// Iterate over artwork, from the starting point.
  pub fn iter<'a>(&'a self) -> Result<PatternIterator<'a>> {
    let p = Patch::default();
    Ok(PatternIterator {
      patterns: Some(vec![
        self
          .getkey(&p)
          .ok_or_else(|| PatchError::MissingRule { patch: p.clone() })?,
      ]),
      rules: self,
    })
  }
}

/// An iterator over artworks that the program produces.
pub struct PatternIterator<'a> {
  patterns: Option<Vec<&'a Patch>>,
  rules: &'a PatternSet,
}

impl<'a> Iterator for PatternIterator<'a> {
  type Item = Result<Vec<&'a Patch>>;

  fn next(&mut self) -> Option<Self::Item> {
    let patterns = match self.patterns {
      Some(ref patterns) => Patch::combine(&patterns)
        .split()
        .and_then(|ps| self.rules.transform(&ps.iter().collect::<Vec<_>>())),
      None => {
        return None;
      }
    };

    match patterns {
      Ok(ps) => {
        self.patterns = Some(ps.clone());
        Some(Ok(ps))
      }
      Err(e) => {
        self.patterns = None;
        Some(Err(e))
      }
    }
  }
}

#[cfg(test)]
mod test {

  use super::*;

  use std::collections::HashSet;

  #[test]
  fn parse_patch() {
    let patch: Patch = "../.#".parse().unwrap();
    assert_eq!(patch.size(), 2);
  }

  #[test]
  fn rotate_patch() {
    let patch: Patch = "../.#".parse().unwrap();
    let p90 = patch.rotate();
    assert_eq!(p90, ".#/..".parse::<Patch>().unwrap());
  }

  #[test]
  fn vary_patch() {
    let patch: Patch = "../.#".parse().unwrap();
    let varpatches = patch.variants();
    assert_eq!(varpatches.len(), 4);
    let unique_variants: HashSet<Patch> = varpatches.iter().cloned().collect();
    assert_eq!(unique_variants.len(), 4);
  }

  #[test]
  fn mutate_patches() {
    let raw_patches = vec![
      "#../.../...".parse().unwrap(),
      "..#/.../...".parse().unwrap(),
      ".../.../#..".parse().unwrap(),
      ".../.../..#".parse().unwrap(),
    ];
    let patches = raw_patches.iter().collect::<Vec<_>>();

    let mutated = Patch::mutate(&patches).unwrap();
    assert_eq!(mutated.len(), 9);

    {
      let expected = vec![
        "#./..".parse::<Patch>().unwrap(),
        "../..".parse::<Patch>().unwrap(),
        ".#/..".parse::<Patch>().unwrap(),
        "../..".parse::<Patch>().unwrap(),
        "../..".parse::<Patch>().unwrap(),
        "../..".parse::<Patch>().unwrap(),
        "../#.".parse::<Patch>().unwrap(),
        "../..".parse::<Patch>().unwrap(),
        "../.#".parse::<Patch>().unwrap(),
      ];
      assert_eq!(mutated, expected);
    }
  }

  #[test]
  fn mutate_vs_split_and_combine() {
    let raw_patches = vec![
      "#../.../...".parse().unwrap(),
      "..#/.../...".parse().unwrap(),
      ".../.../#..".parse().unwrap(),
      ".../.../..#".parse().unwrap(),
    ];
    let patches = raw_patches.iter().collect::<Vec<_>>();

    let new_patches_mutated = Patch::mutate(&patches).unwrap();
    let new_patches_combine_split = Patch::combine(&patches).split().unwrap();
    assert_eq!(new_patches_mutated, new_patches_combine_split);
  }

  #[test]
  fn split_and_combine_patches() {
    let patch = "#..#/..../..../#..#".parse::<Patch>().unwrap();

    assert_eq!(Patch::combine(&vec![&patch]), patch);

    let mutated = Patch::combine(&vec![&patch]).split().unwrap();

    {
      let expected = vec![
        "#./..".parse::<Patch>().unwrap(),
        ".#/..".parse::<Patch>().unwrap(),
        "../#.".parse::<Patch>().unwrap(),
        "../.#".parse::<Patch>().unwrap(),
      ];
      assert_eq!(mutated, expected);
    }
  }

  use test::Bencher;

  #[test]
  fn parse_rules() {
    let mut rules = PatternSet::new();
    rules.insert("../.# => ##./#../...").unwrap();
    rules.insert(".#./..#/### => #..#/..../..../#..#").unwrap();
  }

  #[test]
  fn example_exploded() {
    let mut rules = PatternSet::new();
    rules.insert("../.# => ##./#../...").unwrap();
    rules.insert(".#./..#/### => #..#/..../..../#..#").unwrap();

    let first_patch = Patch::default();
    let mut patch = &first_patch;

    patch = rules.get(patch).unwrap();
    assert_eq!(patch, &"#..#/..../..../#..#".parse::<Patch>().unwrap());

    let mutated = Patch::mutate(&vec![patch]).unwrap();

    {
      let expected = vec![
        "#./..".parse::<Patch>().unwrap(),
        ".#/..".parse::<Patch>().unwrap(),
        "../#.".parse::<Patch>().unwrap(),
        "../.#".parse::<Patch>().unwrap(),
      ];
      assert_eq!(mutated, expected);
    }

    let transformed = rules
      .transform(&mutated.iter().collect::<Vec<_>>())
      .unwrap();
    {
      let expected = vec![
        "##./#../...".parse::<Patch>().unwrap(),
        "##./#../...".parse::<Patch>().unwrap(),
        "##./#../...".parse::<Patch>().unwrap(),
        "##./#../...".parse::<Patch>().unwrap(),
      ];
      assert_eq!(transformed, expected.iter().collect::<Vec<_>>());
    }
    assert_eq!(on(&transformed), 12);
  }

  #[test]
  fn iterator() {
    let mut rules = PatternSet::new();
    rules.insert("../.# => ##./#../...").unwrap();
    rules.insert(".#./..#/### => #..#/..../..../#..#").unwrap();
    let patches = rules.iter().unwrap().nth(0).unwrap().unwrap();
    assert_eq!(patches[0], &"#..#/..../..../#..#".parse::<Patch>().unwrap());
    assert_eq!(patches.len(), 1);
  }

  #[test]
  fn example_iterator() {
    let mut rules = PatternSet::new();
    rules.insert("../.# => ##./#../...").unwrap();
    rules.insert(".#./..#/### => #..#/..../..../#..#").unwrap();

    assert_eq!(on(&rules.iter().unwrap().nth(1).unwrap().unwrap()), 12);
  }

  #[bench]
  fn mutate_bench(b: &mut Bencher) {
    let raw_patches = vec![
      "#../.../...".parse().unwrap(),
      "..#/.../...".parse().unwrap(),
      ".../.../#..".parse().unwrap(),
      ".../.../..#".parse().unwrap(),
    ];
    let patches = raw_patches.iter().collect::<Vec<_>>();
    b.iter(||{
      Patch::mutate(&patches)
    });
  }
  
  #[bench]
  fn split_combine_bench(b: &mut Bencher) {
    let raw_patches = vec![
      "#../.../...".parse().unwrap(),
      "..#/.../...".parse().unwrap(),
      ".../.../#..".parse().unwrap(),
      ".../.../..#".parse().unwrap(),
    ];
    let patches = raw_patches.iter().collect::<Vec<_>>();
    b.iter(||{
      Patch::combine(&patches).split().unwrap()
    });
  }
}
