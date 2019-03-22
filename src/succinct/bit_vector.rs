mod bit_vector;
mod bit_vector_builder;
mod bit_vector_string;

use std::collections::HashSet;
use super::internal_data_structure::raw_bit_vector::RawBitVector;

/// Succinct bit vector.
///
/// # Complexity
/// When the length of a `BitVector` is `N`:
///
/// |                  | `build()` (Implemented in `BitVectorBuilder::build()`) | `access()` | `rank()` | `select()` |
/// |------------------|--------------------------------------------------------|------------|----------|------------|
/// | Time-complexity  | _O(N)_                                                 | _O(1)_     | _O(1)_   | _O(log N)_ |
/// | Space-complexity | _N + o(N)_                                             | _0_        | _o(N)_   | _o(N)_     |
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
