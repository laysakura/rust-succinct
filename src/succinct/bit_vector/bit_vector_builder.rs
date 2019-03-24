use std::collections::HashSet;
use super::{BitVector, BitVectorBuilder, BitVectorSeed, BitVectorString};
use crate::succinct::internal_data_structure::raw_bit_vector::RawBitVector;
use crate::succinct::internal_data_structure::popcount_table::PopcountTable;

impl super::BitVectorBuilder {
    pub fn from_length(length: u64) -> BitVectorBuilder {
        BitVectorBuilder { seed: BitVectorSeed::Length(length), bits_set: HashSet::new() }
    }

    pub fn from_str(bit_vector_str: BitVectorString) -> BitVectorBuilder {
        BitVectorBuilder { seed: BitVectorSeed::Str(bit_vector_str), bits_set: HashSet::new() }
    }

    pub fn set_bit(&mut self, i: u64) -> &mut BitVectorBuilder {
        self.bits_set.insert(i);
        self
    }

    pub fn build(&self) -> BitVector {
        let mut rbv = match &self.seed {
            BitVectorSeed::Length(n) => RawBitVector::from_length(*n),
            BitVectorSeed::Str(bvs) => RawBitVector::from_str(bvs),
        };
        for bit in &self.bits_set { rbv.set_bit(*bit) }

        let n = rbv.length();

        // Create chunks
        let chunk_size: u16 = super::chunk_size(n);
        let chunks_cnt: u64 = n / (chunk_size as u64) + if n % (chunk_size as u64) == 0 { 0 } else { 1 };  // At max: N / (log N)^2 = 2^64 / 64^2 = 2^(64-12)
        // Each chunk takes 2^64 at max (when every 64 bit is 1 for BitVector of length of 2^64)
        let mut chunks: Vec<u64> = Vec::with_capacity(chunks_cnt as usize);
        for i in 0.. (chunks_cnt as usize) {
            let this_chunk_size: u16 =
                if i as u64 == chunks_cnt - 1 {
                    // When `chunk_size == 6`:
                    //
                    //  000 111 000 11   : rbv
                    // |       |      |  : chunks
                    //
                    // Here, when `i == 1` (targeting on last '00011' chunk),
                    // chunk_size == 5
                    let chunk_size_or_0 = (n % chunk_size as u64) as u16;
                    if chunk_size_or_0 == 0 { chunk_size } else { chunk_size_or_0 }
                } else {
                    chunk_size
                };

            let chunk_rbv = rbv.copy_sub(
                i as u64 * chunk_size as u64,
                this_chunk_size as u64,
            );

            let popcount_in_chunk = chunk_rbv.popcount();
            chunks.push(popcount_in_chunk + if i == 0 { 0 } else { chunks[i - 1] });
        }

        // Create blocks
        let block_size: u8 = super::block_size(n);
        let blocks_cnt = n / (block_size as u64) + if n % (block_size as u64) == 0 { 0 } else { 1 };
        // Each block takes (log 2^64)^2 = 64^2 = 2^16 at max (when every bit in a chunk is 1 for BitVector of length of 2^64)
        let mut blocks: Vec<u16> = Vec::with_capacity(blocks_cnt as usize);
        for i in 0.. (chunks_cnt as usize) {
            for j in 0.. ((chunk_size / block_size as u16) as usize) {
                let i_rbv = i as u64 * chunk_size as u64 + j as u64 * block_size as u64;
                if i_rbv >= n { break; }

                let this_block_size: u8 =
                    if i as u64 == chunks_cnt - 1 && j as u64 == blocks_cnt - 1 {
                        // When `chunk_size == 6` and `block_size == 3`:
                        //
                        //  000 111 000 11   : rbv in blocks
                        // |       |      |  : chunks
                        //
                        // Here, when `i == 1` & `j == 1` (targeting on last '11' block),
                        // block_size == 2
                        let block_size_or_0 = (n % block_size as u64) as u8;
                        if block_size_or_0 == 0 { block_size } else { block_size_or_0 }
                    } else {
                        block_size
                    };

                let block_rbv = rbv.copy_sub(i_rbv, this_block_size as u64);

                let popcount_in_block = block_rbv.popcount() as u16;
                blocks.push(popcount_in_block + if j == 0 { 0 } else { blocks[i * chunk_size as usize + j - 1] });
            }
        }

        // Create popcount table
        let popcount_table = PopcountTable::new(block_size);

        BitVector { n, rbv, chunks, blocks, popcount_table }
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
