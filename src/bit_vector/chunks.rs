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
                // chunk_size == 5
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
