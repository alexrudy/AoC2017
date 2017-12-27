#![feature(slice_rotate)]
#![feature(iterator_step_by)]
#![feature(test)]

#[doc(no_inline)]
extern crate test;

#[doc(no_inline)]
#[macro_use]
extern crate failure;

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
