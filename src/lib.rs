#![feature(slice_rotate)]
#![feature(test)]

extern crate test;

/// Tools for working with Hexagons
mod hexagons;

/// Tools for working with arbitrary graphs
pub mod graph;

/// The puzzles module contains general puzzle solutions.
pub mod puzzles;

// Knot hashes
pub mod knot;

// Virtual machines
pub mod vm;