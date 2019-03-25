use super::BitVector;

impl BitVector {
    /// Returns `i`-th element of the `BitVector`.
    ///
    /// # Panics
    /// When _`i` >= length of the `BitVector`_.
    pub fn access(&self, i: u64) -> bool { self.rbv.access(i) }

    /// Returns the number of _1_ in _[0, `i`]_ elements of the `BitVector`.
    ///
    /// # Panics
    /// When _`i` >= length of the `BitVector`_.
    /// 
    /// # Implementation detail
    /// 
    /// ```text
    ///  00001000 01000001 00000100 11000000 00100000 00000101 00100000 00010000 001  Raw data (N=67)
    ///                                                           ^
    ///                                                           i = 51
    /// |                  7                    |                12                |  Chunk (size = (log N)^2 = 36)
    ///                                         ^
    ///                                      i_chunk = 1
    /// |0 |1 |1  |2 |2 |3  |3 |4 |6  |6 |6  |7 |0 |0  |0 |2 |3 |3 |4  |4 |4 |5  |5|  Block (size = log N / 2 = 3)
    ///                                                         ^
    ///                                                      i_block = 17
    /// ```
    /// 
    /// 1. Find `i_chunk`. _`i_chunk` = `i` / chunk size_.
    /// 2. Get _rank from chunk = Chunk[`i_chunk` - 1]_.
    /// 3. Find `i_block`. _`i_block` = `i` / block size_.
    /// 4. Get _rank from block = Block[`i_block` - 1]_.
    /// 5. Get inner-block data. _`block_bits` = [`i` - `i` % (block size), `i`]_. `block_bits` must be of _block size_ length, fulfilled with _0_ in right bits.
    /// 6. Calculate _rank of `block_bits`_ in _O(1)_ using a table memonizing _block size_ bit's popcount.
    pub fn rank(&self, i: u64) -> u64 {
        let chunk_size = self.chunk_size();
        let block_size = self.block_size();

        let i_chunk = i / chunk_size as u64;
        let rank_from_chunk = if i_chunk == 0 { 0 } else { self.chunks[i_chunk as usize - 1] };

        let i_block = i / block_size as u64;
        let rank_from_block = 
        if (i_block * block_size as u64) % chunk_size as u64 == 0 { 0 }
        else { self.blocks[i_block as usize - 1] };

        let block_rbv = self.rbv.copy_sub(i - i % block_size as u64, self.block_size() as u64);
        let block_as_u32 = block_rbv.as_u32();
        let bits_to_use_or_0 = ((i + 1) % block_size as u64) as u8;
        let bits_to_use = if bits_to_use_or_0 == 0 { block_size } else { bits_to_use_or_0 };
        let block_bits = block_as_u32 >> (32 - bits_to_use);
        let rank_from_block_bits = self.popcount_table.popcount(block_bits as u64);

        rank_from_chunk + rank_from_block as u64 + rank_from_block_bits as u64
    }

    fn chunk_size(&self) -> u16 {
        super::chunk_size(self.n)
    }

    fn block_size(&self) -> u8 {
        super::block_size(self.n)
    }
}

#[cfg(test)]
mod access_success_tests {
    // well-tested in bit_vector_builder::{builder_from_length_success_tests, builder_from_str_success_tests}
}

#[cfg(test)]
mod access_failure_tests {
    use super::super::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn over_upper_bound() {
        let bv = BitVectorBuilder::from_length(2).build();
        let _ = bv.access(2);
    }
}

#[cfg(test)]
mod rank_success_tests {
    use super::super::{BitVectorBuilder, BitVectorString};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_bv_str, in_i, expected_rank) = $value;
                assert_eq!(
                    BitVectorBuilder::from_str(BitVectorString::new(in_bv_str))
                        .build().rank(in_i),
                    expected_rank);
            }
        )*
        }
    }

    parameterized_tests! {
        rank1_1: ("0", 0, 0),

        rank2_1: ("00", 0, 0),
        rank2_2: ("00", 1, 0),

        rank3_1: ("01", 0, 0),
        rank3_2: ("01", 1, 1),

        rank4_1: ("10", 0, 1),
        rank4_2: ("10", 1, 1),

        rank5_1: ("11", 0, 1),
        rank5_2: ("11", 1, 2),

        rank6_1: ("10010", 0, 1),
        rank6_2: ("10010", 1, 1),
        rank6_3: ("10010", 2, 1),
        rank6_4: ("10010", 3, 2),
        rank6_5: ("10010", 4, 2),
    }
    // Tested more in tests/ (integration test)
}

#[cfg(test)]
mod rank_failure_tests {
    use super::super::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn rank_over_upper_bound() {
        let bv = BitVectorBuilder::from_length(2).build();
        let _ = bv.rank(2);
    }
}
