//! # Succinct.rs
//!
//! Succinct.rs is a library to provide succinct data structures with _simple API_ and _high performance_.
//!
//! See [README](https://github.com/laysakura/succinct.rs/blob/master/README.md) for more about usage and features.

pub use bit_vector::{BitVector, BitVectorBuilder, BitVectorString};

pub mod bit_vector;
mod internal_data_structure;
