#!/usr/bin/env bash
day=$1

echo "Making input for day$day"
mkdir -p "puzzles/$day/"
pbpaste > "puzzles/$day/input.txt"
