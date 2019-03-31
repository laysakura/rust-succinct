use super::Chunks;
use crate::internal_data_structure::raw_bit_vector::RawBitVector;

impl super::Chunks {
    /// Constructor.
    pub fn new(rbv: &RawBitVector) -> Chunks {
        let n = rbv.length();
        let chunk_size: u16 = Chunks::calc_chunk_size(n);
        let chunks_cnt: u64 =
            n / (chunk_size as u64) + if n % (chunk_size as u64) == 0 { 0 } else { 1 }; // At max: N / (log N)^2 = 2^64 / 64^2 = 2^(64-12)
                                                                                        // Each chunk takes 2^64 at max (when every 64 bit is 1 for BitVector of length of 2^64)
        let mut chunks: Vec<u64> = Vec::with_capacity(chunks_cnt as usize);
        for i in 0..(chunks_cnt as usize) {
            let this_chunk_size: u16 = if i as u64 == chunks_cnt - 1 {
                // When `chunk_size == 6`:
                //
                //  000 111 000 11   : rbv
                // |       |      |  : chunks
                //
                // Here, when `i == 1` (targeting on last '00011' chunk),
                // `this_chunk_size == 5`
                let chunk_size_or_0 = (n % chunk_size as u64) as u16;
                if chunk_size_or_0 == 0 {
                    chunk_size
                } else {
                    chunk_size_or_0
                }
            } else {
                chunk_size
            };

            let chunk_rbv = rbv.copy_sub(i as u64 * chunk_size as u64, this_chunk_size as u64);

            let popcount_in_chunk = chunk_rbv.popcount();
            chunks.push(popcount_in_chunk + if i == 0 { 0 } else { chunks[i - 1] });
        }

        Chunks {
            chunks,
            chunk_size,
            chunks_cnt,
        }
    }

    /// Returns size of 1 chunk.
    pub fn chunk_size(&self) -> u16 {
        self.chunk_size
    }

    /// Returns count of chunks.
    pub fn chunks_cnt(&self) -> u64 {
        self.chunks_cnt
    }

    /// Returns a content (total rank to go) of i-th chunk.
    ///
    /// # Panics
    /// When _`i` >= `self.chunks_cnt()`_.
    pub fn access(&self, i: u16) -> u64 {
        if i > self.chunk_size {
            panic!(
                "i = {} must be smaller then {} (self.chunks_cnt())",
                i, self.chunks_cnt,
            );
        }
        self.chunks[i as usize]
    }

    fn calc_chunk_size(n: u64) -> u16 {
        let lg2 = (n as f64).log2() as u16;
        let sz = lg2 * lg2;
        if sz == 0 {
            1
        } else {
            sz
        }
    }
}

#[cfg(test)]
mod new_success_tests {
    use super::super::BitVectorString;
    use super::Chunks;
    use crate::internal_data_structure::raw_bit_vector::RawBitVector;

    struct Input<'a> {
        in_s: &'a str,
        expected_chunk_size: u16,
        expected_chunks: &'a Vec<u64>,
    }

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input: Input = $value;
                let rbv = RawBitVector::from_str(&BitVectorString::new(input.in_s));
                let chunks = Chunks::new(&rbv);

                assert_eq!(chunks.chunk_size(), input.expected_chunk_size);
                assert_eq!(chunks.chunks_cnt(), input.expected_chunks.len() as u64);
                for (i, expected_chunk) in input.expected_chunks.iter().enumerate() {
                    let chunk = chunks.access(i as u16);
                    assert_eq!(chunk, *expected_chunk);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1: Input {
            in_s: "0", // N = 1, (log_2(N))^2 = 1
            expected_chunk_size: 1,
            expected_chunks: &vec!(0)
        },
        t2: Input {
            in_s: "1", // N = 1, (log_2(N))^2 = 1
            expected_chunk_size: 1,
            expected_chunks: &vec!(1)
        },
        t3: Input {
            in_s: "0111", // N = 2^2, (log_2(N))^2 = 4
            expected_chunk_size: 4,
            expected_chunks: &vec!(3)
        },
        t4: Input {
            in_s: "0111_1101", // N = 2^3, (log_2(N))^2 = 9
            expected_chunk_size: 9,
            expected_chunks: &vec!(6)
        },
        t5: Input {
            in_s: "0111_1101_1", // N = 2^3 + 1, (log_2(N))^2 = 9
            expected_chunk_size: 9,
            expected_chunks: &vec!(7)
        },
        t6: Input {
            in_s: "0111_1101_11", // N = 2^3 + 2, (log_2(N))^2 = 9
            expected_chunk_size: 9,
            expected_chunks: &vec!(7, 8)
        },

        bugfix_11: Input {
            in_s: "11", // N = 2^1, (log_2(N))^2 = 4
            expected_chunk_size: 1,
            expected_chunks: &vec!(1, 2)
        },
        bugfix_11110110_11010101_01000101_11101111_10101011_10100101_01100011_00110100_01010101_10010000_01001100_10111111_00110011_00111110_01110101_11011100: Input {
            in_s: "11110110_11010101_01000101_11101111_10101011_10100101_0__1100011_00110100_01010101_10010000_01001100_10111111_00__110011_00111110_01110101_11011100", // N = 8 * 16 = 2^7, (log_2(N))^2 = 49
            expected_chunk_size: 49,
            expected_chunks: &vec!(30, 53, 72)
        },
    }
}
