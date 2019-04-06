mod bit_vector;
mod bit_vector_builder;
mod bit_vector_string;
mod block;
mod blocks;
mod chunk;
mod chunks;

use super::internal_data_structure::popcount_table::PopcountTable;
use super::internal_data_structure::raw_bit_vector::RawBitVector;
use std::collections::HashSet;

/// Succinct bit vector.
///
/// This class can handle bit sequence of virtually **arbitrary length.**
///
/// In fact, _N_ (`BitVector`'s length) is designed to be limited to: _N <= 2^64_.<br>
/// So you can make a bit vector like `01...` (sequence of _2^64_ '0' or '1'), and then calculate `rank()` and `select()` for that vector.<br>
/// It should be enough for almost all usecases since a binary data of length of _2^64_ consumes _2^24 = 16,777,216_ TB (terabyte), which is hard to handle by state-of-the-art computer architecture.
///
/// # Examples
/// ```
/// extern crate succinct_rs;
/// 
/// use succinct_rs::bit_vector::{BitVectorBuilder, BitVectorString};
/// 
/// // `01001` built by `from_length()` and `set_bit()`
/// let bv = BitVectorBuilder::from_length(5)
///     .set_bit(1)
///     .set_bit(4)
///     .build();
/// 
/// assert_eq!(bv.access(0), false);  // [0]1001; 0th bit is '0' (false)
/// assert_eq!(bv.access(1), true);   // 0[1]001; 1st bit is '1' (true)
/// assert_eq!(bv.access(4), true);   // 0100[1]; 4th bit is '1' (true)
/// 
/// assert_eq!(bv.rank(0), 0);  // [0]1001; Range [0, 0] has no '1'
/// assert_eq!(bv.rank(3), 1);  // [0100]1; Range [0, 3] has 1 '1'
/// assert_eq!(bv.rank(4), 2);  // [01001]; Range [0, 4] has 2 '1's
/// 
/// assert_eq!(bv.select(0), Some(0)); // []01001; Minimum `i` where range [0, i] has 0 '1's is `i=0`
/// assert_eq!(bv.select(1), Some(1)); // 0[1]001; Minimum `i` where range [0, i] has 1 '1's is `i=1`
/// assert_eq!(bv.select(2), Some(4)); // 0100[1]; Minimum `i` where range [0, i] has 2 '1's is `i=4`
/// assert_eq!(bv.select(3), None);    // There is no `i` where range [0, i] has 3 '1's
/// 
/// // `10010` built by `from_str()`
/// let bv = BitVectorBuilder::from_str(BitVectorString::new("1001_0")).build();  // Tips: BitVectorString::new() ignores '_'.
/// ```
///
/// # Complexity
/// When the length of a `BitVector` is `N`:
///
/// |                  | `build()` <br>(Implemented in `BitVectorBuilder::build()`) | `access()` | `rank()` | `select()` |
/// |------------------|--------------------------------------------------------|------------|----------|------------|
/// | Time-complexity  | _O(N)_                                                 | _O(1)_     | _O(1)_   | _O(log N)_ |
/// | Space-complexity | _N + o(N)_                                             | _0_        | _o(N)_   | _o(N)_     |
///
/// # Implementation detail
///
/// ```text
///  00001000 01000001 00000100 11000000 00100000 00000101 00100000 00010000 001  Raw data (N=67)
/// |                  7                    |                12                |  Chunk (size = (log N)^2 = 36)
/// |0 |1 |1  |2 |2 |3  |3 |4 |6  |6 |6  |7 |0 |0  |0 |2 |3 |3 |4  |4 |4 |5  |5|  Block (size = log N / 2 = 3)
/// ```
///
/// TODO Explain about Chunk, Block, and Table.
///
pub struct BitVector {
    /// Raw data.
    rbv: RawBitVector,

    /// Total _popcount_ of _[0, (last bit of the chunk)]_.
    ///
    /// Each chunk takes _2^64_ at max (when every bit is '1' for BitVector of length of _2^64_).
    /// A `chunk` has `blocks`.
    chunks: Chunks,

    /// Table to calculate inner-block rank() in O(1).
    table: PopcountTable,
}

/// Builder of `succinct::BitVector`.
pub struct BitVectorBuilder {
    seed: BitVectorSeed,
    bits_set: HashSet<u64>,
}

/// Provides validated string representation of `succinct::BitVector`.
pub struct BitVectorString {
    s: String,
}

enum BitVectorSeed {
    Length(u64),
    Str(BitVectorString),
}

/// Collection of `Chunk`.
struct Chunks {
    chunks: Vec<Chunk>,
    chunks_cnt: u64,
}

/// Total _popcount_ of _[0, (last bit of the chunk)]_ of a bit vector.
///
/// Each chunk takes _2^64_ at max (when every bit is '1' for BitVector of length of _2^64_).
struct Chunk {
    value: u64, // popcount
    blocks: Blocks,

    #[allow(dead_code)]
    length: u16,
}

/// Collection of `Block` in a `Chunk`.
struct Blocks {
    blocks: Vec<Block>,
    blocks_cnt: u16,
}

/// Total _popcount_ of _[(first bit of the chunk which the block belongs to), (last bit of the block)]_ of a bit vector.
///
/// Each block takes (log 2^64)^2 = 64^2 = 2^16 at max (when every bit in a chunk is 1 for BitVector of length of 2^64)
struct Block {
    value: u16, // popcount
    length: u8,
}
