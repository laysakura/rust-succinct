mod bit_vector;
mod bit_vector_builder;
mod bit_vector_string;

use std::collections::HashSet;
use super::internal_data_structure::raw_bit_vector::RawBitVector;

pub struct BitVector {
    rbv: RawBitVector,
}

pub struct BitVectorBuilder {
    seed: BitVectorSeed,
    bits_set: HashSet<u64>,
}

pub struct BitVectorString { pub s: String }

enum BitVectorSeed {
    Length(u64),
    Str(BitVectorString),
}
