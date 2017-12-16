#!/usr/bin/env bash
day=$1

echo "Making day$day"
if [[ ! -e "src/bin/day$day.rs" ]]; then
  cat > "src/bin/day$day.rs" <<EOF
extern crate aoc2017;
use aoc2017::puzzles::day$day;

fn main() {
  println!("Day $day solution")
}
EOF
fi
touch "src/puzzles/day$day.rs"

grep "pub mod day$day;" src/puzzles/mod.rs
if [[ $? -eq 0 ]]; then
  echo "Already added Day $day to mod.rs"
else
  echo "" >> src/puzzles/mod.rs
  echo "pub mod day$day;" >> src/puzzles/mod.rs
fi