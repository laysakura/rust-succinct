mod bit_vector;
mod bit_vector_builder;
mod bit_vector_string;

use std::collections::HashSet;
use super::internal_data_structure::raw_bit_vector::RawBitVector;

/// Succinct bit vector.
///
/// This class can handle bit sequence of virtually **arbitrary length.**<br>
/// Theoretically, _N_ (`BitVector`'s length) is limited to: _N <= min(2^65, 2^`(mem::size_of::<usize>())`)_.<br>
/// See [Size limitation coming from internal implementation](#size-limitation-coming-from-internal-implementation) for detail.
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
/// TODO Explain about Chunk, Block, and Table.
///
/// # Size limitation coming from internal implementation
/// Finally, _N_ (`BitVector`'s length) is limited to: _N <= min(2^65, 2^`(mem::size_of::<usize>())`)_
///
/// | `struct` used for implementation | Max size to handle | Limitation to _N_ (`BitVector`'s length) | Reasons for limitation |
/// |----------------------------------|--------------------|------------------------------------------|------------------------|
/// | `BitVectorBuilder` <br>for building `BitVector` instance | - (arbitrary) | - | - |
/// | `BitVectorString` <br>for building `BitVector` instance from string representation | - (arbitrary) | - | - |
/// | `RawBitVector` <br>for internal raw, chunk, and block representation of `BitVector` instance | _2^`(mem::size_of::<usize>())`_ | _N <= 2^`(mem::size_of::<usize>())`_ | Public methods' parameters are typed as `usize`. |
/// | `PopcountTable` <br>for calculating inner-block rank in _O(1)_ | _2^64_ rows of table <br>(containing each _popcount_ of _[0, 2^64 - 1]_) | _Block size <= 64_. <br>Thus, _log N / 2 <= 64_ <br>Thus, _N <= 2^65_ | Each row has _popcount_ for each key. <br>_popcount_ is calculated with `u64::count_ones()` Rust function, which is expected to be compiled to fast hardware popcount instruction. |
///
pub struct BitVector {
    rbv: RawBitVector,
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
