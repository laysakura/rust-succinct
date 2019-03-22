use std::collections::HashSet;
use super::{BitVector, BitVectorBuilder, BitVectorSeed, BitVectorString};
use crate::succinct::internal_data_structure::raw_bit_vector::RawBitVector;

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

        let n = &rbv.length();

        // chunks を作る（chunkは、その要素がpopcountの合計であるものを指す）
        let chunk_size = n.log2() * n.log2();
        let chunk_size = if chunk_size == 0 { 1 } else { chunk_size };
        let chunks_cnt = n / chunk_size + if n % chunk_size == 0 { 0 } else { 1 };
        // Each chunk takes 2^64 at max (when every 64 bit is 1 for BitVector of length of 2^64)
        let chunks: Vec<u64> = Vec::with_capacity(chunks_cnt);
        for i in (0.. chunks_cnt) {
            let chunk_rbv = rbv.copy_sub(
                i * chunk_size,
                if i == chunks_cnt - 1 { n % chunk_size } else { chunk_size }
            );

            let popcount_in_chunk = chunk_rbv.popcount();
            chunks[i] = popcount_in_chunk + if i == 0 { 0 } else { chunks[i - 1] };
        }

        // blocks を作る。
        // （blockの定義からして当然だが）chunkを境として0から数える。それにより空間計算量を節約できる。
        let block_size = n.log2() / 2;
        let block_size = if block_size == 0 { 1 } else { block_size };
        let blocks_cnt = n / block_size + if n % block_size == 0 { 0 } else { 1 };
        // Each block takes (log 2^64)^2 = 64^2 = 2^16 at max (when every bit in a chunk is 1 for BitVector of length of 2^64)
        let blocks: Vec<u16> = Vec::with_capacity(blocks_cnt);
        for i in (0.. chunks_cnt) {
            for j in (0.. blocks_cnt) {
                let block_rbv = rbv.copy_sub(
                    i * chunk_size + j * block_size,
                    if i == chunks_cnt - 1 && j == blocks_cnt - 1 {
                        n % block_size
                    } else {
                        block_size
                    }
                );

                let popcount_in_block = block_rbv.popcount();
                blocks[i * chunk_size + j] = popcount_in_block + if i == 0 { 0 } else { blocks[i - 1] };
            }
        }

        // tableを作る
        let popcount_table = PopcountTable::new(block_size);


        BitVector { chunks, blocks, popcount_table }
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
