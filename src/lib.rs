#![feature(slice_rotate)]
#![feature(iterator_step_by)]
#![feature(test)]

#[doc(no_inline)]
extern crate test;

#[doc(no_inline)]
#[macro_use]
extern crate failure;

mod hexagons;
pub mod graph;
pub mod puzzles;
pub mod knot;
pub mod vm;
