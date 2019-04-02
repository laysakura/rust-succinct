use super::{BitVector, Blocks, Chunks};

impl BitVector {
    /// Returns `i`-th element of the `BitVector`.
    ///
    /// # Panics
    /// When _`i` >= length of the `BitVector`_.
    pub fn access(&self, i: u64) -> bool {
        self.rbv.access(i)
    }

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
    ///                chunk_left            i_chunk = 1      chunk_right
    ///
    /// |0 |1 |1  |2 |2 |3  |3 |4 |6  |6 |6  |7 |0 |0  |0 |2 |3 |3 |4  |4 |4 |5  |5|  Block (size = log N / 2 = 3)
    ///                                                         ^
    ///                                                      i_block = 17
    ///                                              block_left | block_right
    /// ```
    ///
    /// 1. Find `i_chunk`. _`i_chunk` = `i` / `chunk_size`_.
    /// 2. Get _`chunk_left` = Chunks[`i_chunk` - 1]_ only if _`i_chunk` > 0_.
    /// 3. Get _rank from chunk_left_ if `chunk_left` exists.
    /// 4. Get _`chunk_right` = Chunks[`i_chunk`]_.
    /// 5. Find `i_block`. _`i_block` = (`i` - `i_chunk` * `chunk_size`) / block size_.
    /// 6. Get _`block_left` = `chunk_right.blocks`[ `i_block` - 1]`_ only if _`i_block` > 0_.
    /// 7. Get _rank from block_left_ if `block_left` exists.
    /// 8. Get inner-block data. _`block_bits` = [(`i` - i_chunk * chunk_size) % block_size, `i`]_. `block_bits` must be of _block size_ length, fulfilled with _0_ in right bits.
    /// 9. Calculate _rank of `block_bits`_ in _O(1)_ using a table memonizing _block size_ bit's popcount.
    pub fn rank(&self, i: u64) -> u64 {
        let n = self.rbv.length();
        assert!(i < n);
        let chunk_size = Chunks::calc_chunk_size(n);
        let block_size = Blocks::calc_block_size(n);

        // 1.
        let i_chunk = i / chunk_size as u64;

        // 3.
        let rank_from_chunk = if i_chunk == 0 {
            0
        } else {
            // 2., 3.
            let chunk_left = self.chunks.access(i_chunk - 1);
            chunk_left.value()
        };

        // 4.
        let chunk_right = self.chunks.access(i_chunk);

        // 5.
        let i_block = (i - i_chunk * chunk_size as u64) / block_size as u64;

        // 7.
        let rank_from_block = if i_block == 0 {
            0
        } else {
            // 6., 7.
            let block_left = chunk_right.blocks.access(i_block - 1);
            block_left.value()
        };

        // 8.
        let block_right = chunk_right.blocks.access(i_block);
        let pos_block_start = i_chunk * chunk_size as u64 + i_block * block_size as u64;
        assert!(i - pos_block_start < block_right.length() as u64);
        let block_right_rbv = self.rbv.copy_sub(pos_block_start, block_size as u64);
        let block_right_as_u32 = block_right_rbv.as_u32();
        let bits_to_use = i - pos_block_start + 1;
        let block_bits = block_right_as_u32 >> (32 - bits_to_use);
        let rank_from_table = self.table.popcount(block_bits as u64);

        // 9.
        rank_from_chunk + rank_from_block as u64 + rank_from_table as u64
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

        bugfix_11110110_11010101_01000101_11101111_10101011_10100101_01100011_00110100_01010101_10010000_01001100_10111111_00110011_00111110_01110101_11011100: (
            "11110110_11010101_01000101_11101111_10101011_10100101_01100011_00110100_01010101_10010000_01001100_10111111_00110011_00111110_01110101_11011100",
            49, 31,
        ),
        bugfix_10100001_01010011_10101100_11100001_10110010_10000110_00010100_01001111_01011100_11010011_11110000_00011010_01101111_10101010_11000111_0110011: (
            "10100001_01010011_10101100_11100001_10110010_10000110_00010100_01001111_01011100_11010011_11110000_00011010_01101111_10101010_11000111_0110011",
            111, 55,
        ),
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
