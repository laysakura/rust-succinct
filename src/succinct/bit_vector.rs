mod bit_vector;
mod bit_vector_builder;
mod bit_vector_string;

use std::collections::HashSet;
use super::internal_data_structure::raw_bit_vector::RawBitVector;
use super::internal_data_structure::popcount_table::PopcountTable;

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
/// use rust_succinct::succinct::bit_vector::{BitVectorBuilder, BitVectorString};
///
/// // `01001` built by `from_length()` and `set_bit()`
/// let bv = BitVectorBuilder::from_length(5)
///     .set_bit(1)
///     .set_bit(4)
///     .build();
///
/// assert_eq!(bv.access(0), false);  // [0]1001
/// assert_eq!(bv.access(1), true);   // 0[1]001
/// assert_eq!(bv.access(4), true);   // 0100[1]
///
/// assert_eq!(bv.rank(0), 0);  // [0]1001
/// assert_eq!(bv.rank(3), 1);  // [0100]1
/// assert_eq!(bv.rank(4), 2);  // [01001]
///
/// // TODO select() example
///
/// // `10010` built by `from_str()`
/// let bv = BitVectorBuilder::from_str(BitVectorString::new("10010")).build();
///
/// assert_eq!(bv.access(0), true);   // [1]0010
/// assert_eq!(bv.access(1), false);  // 1[0]010
/// assert_eq!(bv.access(4), false);  // 1001[0]
///
/// assert_eq!(bv.rank(0), 1);  // [1]0010
/// assert_eq!(bv.rank(3), 2);  // [1001]0
/// assert_eq!(bv.rank(4), 2);  // [10010]
///
/// // TODO select() example
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
/// |0 |1 |1  |2 |2 |3  |3 |4 |6  |6 |6  |7 |0 |0  |0 |2 |3 |3 |3  |4 |4 |4  |4|  Block (size = log N / 2 = 3)
/// ```
///
/// TODO Explain about Chunk, Block, and Table.
///
pub struct BitVector {
    /// Length.
    n: u64,

    /// Raw data.
    rbv: RawBitVector,

    /// Total _popcount_ of _[0, (last bit of the chunk)]_.
    ///
    /// Each chunk takes _2^64_ at max (when every 64 bit is '1' for BitVector of length of _2^64_).
    chunks: Vec<u64>,

    /// Total _popcount_ of _[(first bit of the chunk where the block belongs to), (last bit of the chunk where the block belongs to)]_.
    ///
    /// Each block takes (log 2^64)^2 = 64^2 = 2^16 at max (when every bit in a chunk is 1 for BitVector of length of 2^64)
    blocks: Vec<u16>,

    /// Table to calculate inner-block rank() in O(1).
    popcount_table: PopcountTable,
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

fn chunk_size(n: u64) -> u16 {
    let lg2 = log2(n) as u16;
    let sz = lg2 * lg2;
    if sz == 0 { 1 } else { sz }
}

fn block_size(n: u64) -> u8 {
    let sz = log2(n) / 2;
    if sz == 0 { 1 } else { sz }
}

fn log2(n: u64) -> u8 {
    (n as f64).log2() as u8
}
