mod bit_vector;
mod bit_vector_builder;

use std::collections::HashSet;
use super::internal_data_structure::raw_bit_vector::RawBitVector;

pub struct BitVector {
    rbv: RawBitVector,
}

pub struct BitVectorBuilder {
    seed: BitVectorSeed,
    bits_set: HashSet<usize>,
}

enum BitVectorSeed {
    Length(usize),
    Str(String),
}
