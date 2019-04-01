use super::{Blocks, Chunks};
use crate::internal_data_structure::raw_bit_vector::RawBitVector;

impl super::Blocks {
    /// Constructor.
    pub fn new(rbv: &RawBitVector, chunks: &Chunks) -> Blocks {
        let n = rbv.length();
        let block_size = Blocks::calc_block_size(n);
        let blocks_cnt: u64 =
            n / (block_size as u64) + if n % (block_size as u64) == 0 { 0 } else { 1 };

        let blocks_in_chunk_cnt = chunks.chunk_size() / block_size as u16;
        // Each block takes (log 2^64)^2 = 64^2 = 2^16 at max (when every bit in a chunk is 1 for BitVector of length of 2^64)
        let mut blocks: Vec<u16> = Vec::with_capacity(blocks_cnt as usize);
        for i in 0..(chunks.chunks_cnt() as usize) {
            for j in 0..((blocks_in_chunk_cnt as u16) as usize) {
                let i_rbv = i as u64 * chunks.chunk_size() as u64 + j as u64 * block_size as u64;
                if i_rbv >= n {
                    break;
                }

                let this_block_size: u8 = if n - i_rbv >= block_size as u64 {
                    block_size
                } else {
                    (n - i_rbv) as u8
                };

                let block_rbv = rbv.copy_sub(i_rbv, this_block_size as u64);

                let popcount_in_block = block_rbv.popcount() as u16;
                blocks.push(
                    popcount_in_block
                        + if j == 0 {
                            0
                        } else {
                            blocks[i * blocks_in_chunk_cnt as usize + j - 1]
                        },
                );
            }
        }

        Blocks {
            blocks,
            block_size,
            blocks_cnt,
        }
    }

    /// Returns size of 1 block.
    pub fn block_size(&self) -> u8 {
        self.block_size
    }

    /// Returns a content (total rank to go) of i-th block.
    ///
    /// # Panics
    /// When _`i` >= `self.blocks_cnt()`_.
    pub fn access(&self, i: u64) -> u16 {
        if i > self.blocks_cnt {
            panic!(
                "i = {} must be smaller then {} (self.blocks_cnt())",
                i, self.blocks_cnt,
            );
        }
        self.blocks[i as usize]
    }

    fn calc_block_size(n: u64) -> u8 {
        let lg2 = (n as f64).log2() as u8;
        let sz = lg2 / 2;
        if sz == 0 {
            1
        } else {
            sz
        }
    }
}
