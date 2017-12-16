#!/usr/bin/env bash
day=$1
echo "Making day$day"
touch "src/bin/day$day.rs"
touch "src/puzzles/day$day.rs"
grep "pub mod day$day;" src/puzzles/mod.rs
if [[ $? -eq 0 ]]; then
  echo "Already added Day $day to mod.rs"
else
  echo "" >> src/puzzles/mod.rs
  echo "pub mod day$day;" >> src/puzzles/mod.rs
fi