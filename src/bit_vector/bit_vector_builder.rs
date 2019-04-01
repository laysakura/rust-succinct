use super::{BitVector, BitVectorBuilder, BitVectorSeed, BitVectorString, Blocks, Chunks};
use crate::internal_data_structure::popcount_table::PopcountTable;
use crate::internal_data_structure::raw_bit_vector::RawBitVector;
use std::collections::HashSet;

impl super::BitVectorBuilder {
    /// Prepares a bit vector of `length`, willed with 0.
    ///
    /// # Panics
    /// When _`length` == 0_.
    pub fn from_length(length: u64) -> BitVectorBuilder {
        if length == 0 {
            panic!("length must be > 0.")
        };

        BitVectorBuilder {
            seed: BitVectorSeed::Length(length),
            bits_set: HashSet::new(),
        }
    }

    /// Prepares a bit vector from `BitVectorString` representation.
    pub fn from_str(bit_vector_str: BitVectorString) -> BitVectorBuilder {
        BitVectorBuilder {
            seed: BitVectorSeed::Str(bit_vector_str),
            bits_set: HashSet::new(),
        }
    }

    /// Set 1 to i-th bit.
    ///
    /// # Panics
    /// When _`i` >= Length of bit vector to build_.
    pub fn set_bit(&mut self, i: u64) -> &mut BitVectorBuilder {
        let length = match &self.seed {
            BitVectorSeed::Length(n) => *n,
            BitVectorSeed::Str(bvs) => bvs.s.len() as u64,
        };
        if i >= length {
            panic!(
                "`i` must be smaller than {} (length of bit vector to build)",
                length
            )
        };

        self.bits_set.insert(i);
        self
    }

    /// Build `succinct::BitVector` in _O(N)_ time (where _N_ is the length of the bit vector to build).
    pub fn build(&self) -> BitVector {
        let mut rbv = match &self.seed {
            BitVectorSeed::Length(n) => RawBitVector::from_length(*n),
            BitVectorSeed::Str(bvs) => RawBitVector::from_str(bvs),
        };
        for bit in &self.bits_set {
            rbv.set_bit(*bit)
        }

        let chunks = Chunks::new(&rbv);

        // Create blocks
        let blocks = Blocks::new(&rbv);
        let block_size = blocks.block_size();

        // Create popcount table
        let table = PopcountTable::new(block_size);

        BitVector {
            rbv,
            chunks,
            blocks,
            table,
        }
    }
}

#[cfg(test)]
mod builder_from_length_success_tests {
    use super::BitVectorBuilder;

    struct IndexBitPair(u64, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_length, index_bit_pairs) = $value;
                let bv = BitVectorBuilder::from_length(in_length).build();
                for IndexBitPair(i, bit) in index_bit_pairs {
                    assert_eq!(bv.access(i), bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1: (1, vec!(
            IndexBitPair(0, false),
        )),
        t2: (2, vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, false),
        )),
        t8: (8, vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, false),
            IndexBitPair(2, false),
            IndexBitPair(3, false),
            IndexBitPair(4, false),
            IndexBitPair(5, false),
            IndexBitPair(6, false),
            IndexBitPair(7, false),
        )),
        t9: (9, vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, false),
            IndexBitPair(2, false),
            IndexBitPair(3, false),
            IndexBitPair(4, false),
            IndexBitPair(5, false),
            IndexBitPair(6, false),
            IndexBitPair(7, false),
            IndexBitPair(8, false),
        )),
        t2_pow_16: (1 << 16, vec!(
            IndexBitPair(0, false),
            IndexBitPair((1 << 16) - 1, false),
        )),
        t2_pow_16_p1: ((1 << 16) + 1, vec!(
            IndexBitPair(0, false),
            IndexBitPair(1 << 16, false),
        )),
        t2_pow_16_m1: ((1 << 16) - 1, vec!(
            IndexBitPair(0, false),
            IndexBitPair((1 << 16) - 2, false),
        )),
    }
}

#[cfg(test)]
mod builder_from_length_failure_tests {
    use super::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn empty() {
        let _ = BitVectorBuilder::from_length(0).build();
    }
}

#[cfg(test)]
mod builder_from_str_success_tests {
    use super::{BitVectorBuilder, BitVectorString};

    struct IndexBitPair(u64, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, index_bit_pairs) = $value;
                let bv = BitVectorBuilder::from_str(BitVectorString::new(in_s)).build();
                for IndexBitPair(i, bit) in index_bit_pairs {
                    assert_eq!(bv.access(i), bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("0", vec!(
            IndexBitPair(0, false),
        )),
        t1_2: ("1", vec!(
            IndexBitPair(0, true),
        )),

        t2_1: ("00", vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, false),
        )),
        t2_2: ("01", vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, true),
        )),
        t2_3: ("10", vec!(
            IndexBitPair(0, true),
            IndexBitPair(1, false),
        )),
        t2_4: ("11", vec!(
            IndexBitPair(0, true),
            IndexBitPair(1, true),
        )),

        t8_1: ("00000000", vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, false),
            IndexBitPair(2, false),
            IndexBitPair(3, false),
            IndexBitPair(4, false),
            IndexBitPair(5, false),
            IndexBitPair(6, false),
            IndexBitPair(7, false),
        )),
        t8_2: ("11111111", vec!(
            IndexBitPair(0, true),
            IndexBitPair(1, true),
            IndexBitPair(2, true),
            IndexBitPair(3, true),
            IndexBitPair(4, true),
            IndexBitPair(5, true),
            IndexBitPair(6, true),
            IndexBitPair(7, true),
        )),
        t8_3: ("01010101", vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, true),
            IndexBitPair(2, false),
            IndexBitPair(3, true),
            IndexBitPair(4, false),
            IndexBitPair(5, true),
            IndexBitPair(6, false),
            IndexBitPair(7, true),
        )),

        t9_1: ("000000000", vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, false),
            IndexBitPair(2, false),
            IndexBitPair(3, false),
            IndexBitPair(4, false),
            IndexBitPair(5, false),
            IndexBitPair(6, false),
            IndexBitPair(7, false),
            IndexBitPair(8, false),
        )),
        t9_2: ("111111111", vec!(
            IndexBitPair(0, true),
            IndexBitPair(1, true),
            IndexBitPair(2, true),
            IndexBitPair(3, true),
            IndexBitPair(4, true),
            IndexBitPair(5, true),
            IndexBitPair(6, true),
            IndexBitPair(7, true),
            IndexBitPair(8, true),
        )),
        t9_3: ("101010101", vec!(
            IndexBitPair(0, true),
            IndexBitPair(1, false),
            IndexBitPair(2, true),
            IndexBitPair(3, false),
            IndexBitPair(4, true),
            IndexBitPair(5, false),
            IndexBitPair(6, true),
            IndexBitPair(7, false),
            IndexBitPair(8, true),
        )),
    }
}

#[cfg(test)]
mod builder_from_str_failure_tests {
    // well-tested in BitVectorString
}

#[cfg(test)]
mod set_bit_success_tests {
    use super::{BitVectorBuilder, BitVectorString};

    struct IndexBitPair(u64, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, bits_to_set, index_bit_pairs) = $value;
                let mut builder = BitVectorBuilder::from_str(BitVectorString::new(in_s));

                for i in bits_to_set { builder.set_bit(i); }
                let bv = builder.build();

                for IndexBitPair(i, bit) in index_bit_pairs {
                    assert_eq!(bv.access(i), bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("0", vec!(),
               vec!(
                    IndexBitPair(0, false),
                   )),
        t1_2: ("0", vec!(0),
               vec!(
                    IndexBitPair(0, true),
                   )),
        t1_3: ("0", vec!(0, 0),
               vec!(
                    IndexBitPair(0, true),
                   )),
        t1_4: ("1", vec!(0),
               vec!(
                    IndexBitPair(0, true),
                   )),

        t8_1: ("00000000", vec!(),
               vec!(
                    IndexBitPair(0, false),
                    IndexBitPair(1, false),
                    IndexBitPair(2, false),
                    IndexBitPair(3, false),
                    IndexBitPair(4, false),
                    IndexBitPair(5, false),
                    IndexBitPair(6, false),
                    IndexBitPair(7, false),
                   )),
        t8_2: ("00000000", vec!(0, 2, 4, 6),
               vec!(
                    IndexBitPair(0, true),
                    IndexBitPair(1, false),
                    IndexBitPair(2, true),
                    IndexBitPair(3, false),
                    IndexBitPair(4, true),
                    IndexBitPair(5, false),
                    IndexBitPair(6, true),
                    IndexBitPair(7, false),
                   )),

        t9_1: ("000000000", vec!(),
               vec!(
                    IndexBitPair(0, false),
                    IndexBitPair(1, false),
                    IndexBitPair(2, false),
                    IndexBitPair(3, false),
                    IndexBitPair(4, false),
                    IndexBitPair(5, false),
                    IndexBitPair(6, false),
                    IndexBitPair(7, false),
                    IndexBitPair(8, false),
                   )),
        t9_2: ("000000000", vec!(0, 2, 4, 6, 8),
               vec!(
                    IndexBitPair(0, true),
                    IndexBitPair(1, false),
                    IndexBitPair(2, true),
                    IndexBitPair(3, false),
                    IndexBitPair(4, true),
                    IndexBitPair(5, false),
                    IndexBitPair(6, true),
                    IndexBitPair(7, false),
                    IndexBitPair(8, true),
                   )),
    }
}

#[cfg(test)]
mod builder_set_bit_failure_tests {
    use super::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn set_bit_over_upper_bound() {
        let _ = BitVectorBuilder::from_length(2).set_bit(2).build();
    }
}
