//! # succinct.rs - Succinct data structures for rust.
//!
//! Currently, this library provides **succinct bit vector** (**succinct indexable dictionary**).

pub use bit_vector::{BitVector, BitVectorBuilder, BitVectorString};

pub mod bit_vector;
mod internal_data_structure;
