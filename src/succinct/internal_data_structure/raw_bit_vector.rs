use std::fmt;
use crate::succinct::bit_vector::BitVectorString;

#[derive(PartialEq, Eq, Debug)]
/// Bit vector of arbitrary length (actually the length is limited to _[1, 2^64)_).
pub struct RawBitVector {
    byte_vec: Vec<u8>,
    last_byte_len: u8,
}

impl RawBitVector {
    /// Makes a bit vector of `length`, willed with 0.
    ///
    /// # Panics
    /// When _`length` == 0_.
    pub fn from_length(length: u64) -> RawBitVector {
        if length == 0 { panic!("length must be > 0.") };

        let last_byte_len_or_0 = (length % 8) as u8;
        let last_byte_len = if last_byte_len_or_0 == 0 { 8 } else { last_byte_len_or_0 };

        RawBitVector {
            byte_vec: vec![0; ((length - 1) / 8 + 1) as usize],
            last_byte_len,
        }
    }

    /// Makes a bit vector from `BitVectorString` representation.
    pub fn from_str(bit_vector_str: &BitVectorString) -> RawBitVector {
        let mut rbv = RawBitVector::from_length(bit_vector_str.s.len() as u64);
        for (i, c) in bit_vector_str.s.chars().enumerate() {
            if c == '1' { rbv.set_bit(i as u64); };
        }
        rbv
    }

    /// Returns i-th bit.
    ///
    /// # Panics
    /// When _`i` >= `self.length()`_.
    pub fn access(&self, i: u64) -> bool {
        self.validate_index(i);
        let byte = self.byte_vec[(i / 8) as usize];
        match i % 8 {
            0 => byte & 0b1000_0000 != 0,
            1 => byte & 0b0100_0000 != 0,
            2 => byte & 0b0010_0000 != 0,
            3 => byte & 0b0001_0000 != 0,
            4 => byte & 0b0000_1000 != 0,
            5 => byte & 0b0000_0100 != 0,
            6 => byte & 0b0000_0010 != 0,
            7 => byte & 0b0000_0001 != 0,
            _ => panic!("never happen")
        }
    }

    /// Set 1 to i-th bit.
    ///
    /// # Panics
    /// When _`i` >= `self.length()`_.
    pub fn set_bit(&mut self, i: u64) {
        self.validate_index(i);
        let byte = self.byte_vec[(i / 8) as usize];
        self.byte_vec[(i / 8) as usize] = match i % 8 {
            0 => byte | 0b1000_0000,
            1 => byte | 0b0100_0000,
            2 => byte | 0b0010_0000,
            3 => byte | 0b0001_0000,
            4 => byte | 0b0000_1000,
            5 => byte | 0b0000_0100,
            6 => byte | 0b0000_0010,
            7 => byte | 0b0000_0001,
            _ => panic!("never happen")
        }
    }

    /// Returns length.
    pub fn length(&self) -> u64 {
        (self.byte_vec.len() as u64 - 1) * 8 + (self.last_byte_len as u64)
    }

    /// Returns popcount of whole this BitVector.
    pub fn popcount(&self) -> u64 {
        self.byte_vec.iter().fold(0, |popcnt: u64, byte| byte.count_ones() as u64 + popcnt)
    }

    /// Makes another RawBitVector from _[`i`, `i` + `size`)_ of self.
    ///
    /// # Panics
    /// When:
    ///
    /// - _`i` + `size` >= `self.length()`_
    /// - _`size` == 0_
    pub fn copy_sub(&self, i: u64, size: u64) -> RawBitVector {
        if i + size > self.length() { panic!("i + size must be <= self.length(); i = {}, size = {}, self.length() = {}", i, size, self.length()) };
        if size == 0 { panic!("length must be > 0") };

        let mut sub_byte_vec: Vec<u8> = Vec::with_capacity(size as usize / 8 + 1);

        // Memo for implementation: 
        // Assume self.byte_vec == 00000000 11111111 00000000 11111
        //
        // 00000000 11111111 00000000 11111
        //        ^                     ^
        //        i=7                   i+size = 7+19 = 26
        //      (start)                 (end)
        //
        // Copy [start, end).
        //
        // Use j for iterator:
        //   j = start, start+8, ..., start+16 (>= end - 8)
        let start = i;
        let end = start + size;

        for j in (start.. end).step_by(8) {
            if j % 8 == 0 {
                // Copy 1~8 bits from a single byte in self.byte_vec.
                let byte = self.byte_vec[j as usize / 8];
                let right_bits_to_discard = if end - j >= 8 { 0 } else { 8 - (end - j) };
                let copied_byte = byte >> right_bits_to_discard << right_bits_to_discard;
                sub_byte_vec.push(copied_byte);
            } else {
                // Copy 1~8 bits from 2 byte in self.byte_vec (second byte can be a sentinel).
                let byte1 = self.byte_vec[j as usize / 8];
                let byte2 = if (j as usize + 8) / 8 < self.byte_vec.len() {
                    self.byte_vec[(j as usize + 8) / 8]
                } else {
                    0u8
                };
                let left_bits_to_discard_from_byte1 = j % 8;
                let right_bits_to_discard_from_byte2 = 8 - j % 8;
                let copied_byte = (byte1 << left_bits_to_discard_from_byte1) | (byte2 >> right_bits_to_discard_from_byte2);
                let right_bits_to_discard_from_copied_byte = 
                    if end - j >= 8 { 0 } else { 8 - (end - j) };
                let copied_byte = copied_byte >> right_bits_to_discard_from_copied_byte << right_bits_to_discard_from_copied_byte;
                sub_byte_vec.push(copied_byte);
            }
        }

        RawBitVector {
            byte_vec: sub_byte_vec,
            last_byte_len: if size % 8 == 0 { 8 } else { (size % 8) as u8 },
        }
    }

    fn validate_index(&self, i: u64) {
        if i >= self.length() { panic!("`i` must be smaller than {} (length of RawBitVector)", self.length()) };
    }
}

impl fmt::Display for RawBitVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bits_str = self.byte_vec.iter().enumerate().map(|(i, byte)| {
            let byte_s = format!("{: >8}", format!("{:b}", byte)).replace(' ', "0");
            if i < self.byte_vec.len() - 1 {
                byte_s
            } else { 
                byte_s.chars().take(self.last_byte_len as usize).collect::<String>()
             }
        }).collect::<Vec<String>>().concat();

        write!(f, "{}", bits_str)
    }
}

#[cfg(test)]
mod from_length_success_tests {
    use super::RawBitVector;

    struct IndexBitPair(u64, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_length, index_bit_pairs) = $value;
                let rbv = RawBitVector::from_length(in_length);
                for IndexBitPair(i, bit) in index_bit_pairs {
                    assert_eq!(rbv.access(i), bit);
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
mod from_length_failure_tests {
    use super::RawBitVector;

    #[test]
    #[should_panic]
    fn empty() {
        let _ = RawBitVector::from_length(0);
    }
}

#[cfg(test)]
mod from_str_success_tests {
    use crate::succinct::bit_vector::BitVectorString;
    use super::RawBitVector;

    struct IndexBitPair(u64, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, index_bit_pairs) = $value;
                let rbv = RawBitVector::from_str(&BitVectorString::new(in_s));
                for IndexBitPair(i, bit) in index_bit_pairs {
                    assert_eq!(rbv.access(i), bit);
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
mod from_str_failure_tests {
    // well-tested in BitVectorString
}

#[cfg(test)]
mod length_success_tests {
    use super::RawBitVector;

    #[test]
    fn test() {
        for length in 1..= 1 << 16 {
            let rbv = RawBitVector::from_length(length);
            assert_eq!(rbv.length(), length);
        }
     }
}

#[cfg(test)]
mod length_failure_tests {
    // Nothing to do
}

#[cfg(test)]
mod access_success_tests {
    // well-tested in from_length_success_tests & from_str_success_tests
}

#[cfg(test)]
mod access_failure_tests {
    use super::RawBitVector;

    #[test]
    #[should_panic]
    fn over_upper_bound() {
        let rbv = RawBitVector::from_length(2);
        let _ = rbv.access(2);
    }
}

#[cfg(test)]
mod popcount_success_tests {
    use super::{RawBitVector, BitVectorString};
    
    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, expected_popcount) = $value;
                let rbv = RawBitVector::from_str(&BitVectorString::new(in_s));
                assert_eq!(rbv.popcount(), expected_popcount);
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("0", 0),

        t8_1: ("00000000", 0),
        t8_2: ("01010101", 4),
        t8_3: ("10101010", 4),
        t8_4: ("11111111", 8),

        t9_1: ("000000000", 0),
        t9_2: ("010101010", 4),
        t9_3: ("101010101", 5),
        t9_4: ("111111111", 9),
    }
}

#[cfg(test)]
mod popcount_failure_tests {
    // Nothing to do
}

#[cfg(test)]
mod set_bit_success_tests {
    use crate::succinct::bit_vector::BitVectorString;
    use super::RawBitVector;

    struct IndexBitPair(u64, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, bits_to_set, index_bit_pairs) = $value;
                let mut rbv = RawBitVector::from_str(&BitVectorString::new(in_s));

                for i in bits_to_set { rbv.set_bit(i) }

                for IndexBitPair(i, bit) in index_bit_pairs {
                    assert_eq!(rbv.access(i), bit);
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
mod set_bit_failure_tests {
    use super::RawBitVector;

    #[test]
    #[should_panic]
    fn set_bit_over_upper_bound() {
        let mut rbv = RawBitVector::from_length(2);
        rbv.set_bit(2);
    }
}

#[cfg(test)]
mod copy_sub_success_tests {
    use super::{RawBitVector, BitVectorString};
    
    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, i, size, expected_bit_vec) = $value;
                let rbv = RawBitVector::from_str(&BitVectorString::new(s));
                let copied_rbv = rbv.copy_sub(i, size);

                assert_eq!(copied_rbv.length(), expected_bit_vec.len() as u64);
                for (i, expected_bit) in expected_bit_vec.iter().enumerate() {
                    assert_eq!(copied_rbv.access(i as u64), *expected_bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("0", 0, 1, vec![false]),

        t8_1_1: ("01000101", 0, 1, vec![false]),
        t8_1_2: ("01000101", 0, 2, vec![false, true]),
        t8_1_3: ("01000101", 0, 3, vec![false, true, false]),
        t8_1_4: ("01000101", 0, 4, vec![false, true, false, false]),
        t8_1_5: ("01000101", 0, 5, vec![false, true, false, false, false]),
        t8_1_6: ("01000101", 0, 6, vec![false, true, false, false, false, true]),
        t8_1_7: ("01000101", 0, 7, vec![false, true, false, false, false, true, false]),
        t8_1_8: ("01000101", 0, 8, vec![false, true, false, false, false, true, false, true]),

        t8_2_1: ("01000101", 7, 1, vec![true]),

        t9_1_1: ("010001010", 0, 1, vec![false]),
        t9_1_2: ("010001010", 0, 2, vec![false, true]),
        t9_1_3: ("010001010", 0, 3, vec![false, true, false]),
        t9_1_4: ("010001010", 0, 4, vec![false, true, false, false]),
        t9_1_5: ("010001010", 0, 5, vec![false, true, false, false, false]),
        t9_1_6: ("010001010", 0, 6, vec![false, true, false, false, false, true]),
        t9_1_7: ("010001010", 0, 7, vec![false, true, false, false, false, true, false]),
        t9_1_8: ("010001010", 0, 8, vec![false, true, false, false, false, true, false, true]),
        t9_1_9: ("010001010", 0, 9, vec![false, true, false, false, false, true, false, true, false]),

        t9_2_1: ("010001010", 7, 1, vec![true]),
        t9_2_2: ("010001010", 7, 2, vec![true, false]),

        t9_3_1: ("010001010", 8, 1, vec![false]),

        t13_1_1: ("1011001001010", 9, 4, vec![true, false, true, false]),
    }
}

#[cfg(test)]
mod copy_sub_failure_tests {
    use super::{RawBitVector, BitVectorString};
    
    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (s, i, size) = $value;
                let rbv = RawBitVector::from_str(&BitVectorString::new(s));
                let _ = rbv.copy_sub(i, size);
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("0", 0, 0),
        t1_2: ("0", 0, 2),
        t1_3: ("0", 1, 1),

        t8_1_1: ("01000101", 0, 0),
        t8_1_2: ("01000101", 0, 9),

        t8_2_1: ("01000101", 7, 0),
        t8_2_2: ("01000101", 7, 2),

        t9_1_1: ("010001010", 0, 0),
        t9_1_2: ("010001010", 0, 10),

        t9_2_1: ("010001010", 7, 0),
        t9_2_2: ("010001010", 7, 3),

        t9_3_1: ("010001010", 8, 0),
        t9_3_2: ("010001010", 8, 2),
    }
}

#[cfg(test)]
mod copy_sub_fuzzing_tests {
    use super::{RawBitVector, BitVectorString};

    #[test]
    fn test() {
        let samples = 10000;

        fn sub_str(s: &str, i: u64, size: u64) -> String {
            let ss: String = s.chars().skip(i as usize).take(size as usize).collect();
            ss
        }

        for _ in 0.. samples {
            let s = &format!("{:b}", rand::random::<u16>());
            let bvs = BitVectorString::new(s);
            let rbv = RawBitVector::from_str(&bvs);

            for i in 0.. s.len() {
                for size in 1.. (s.len() - i) {
                    let copied_rbv = rbv.copy_sub(i as u64, size as u64);

                    let substr = sub_str(s, i as u64, size as u64);
                    let substr_bvs = BitVectorString::new(&substr);
                    let substr_rbv = RawBitVector::from_str(&substr_bvs);

                    assert_eq!(copied_rbv, substr_rbv,
                        "\nbit vector = {}, RawBitVector::copy_sub(i={}, size={});\nActual:   {}\nExpected: {}",
                        s, i, size, copied_rbv, substr
                    );
                }
            }
        }
    }
}