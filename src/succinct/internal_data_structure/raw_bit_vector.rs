use crate::succinct::bit_vector::BitVectorString;

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
            byte_vec: vec![0; (length / 8 + 1) as usize],
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
        (self.byte_vec.len() as u64 - 1) * 8 + (self.last_byte_len as u64 % 8)
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

        // Memo for implementation: Assume `self.byte_vec == 00000000 11111111 0000`
        for i_byte_vec in (i as usize / 8)..= (i + size) as usize / 8 {
            let sub_byte: u8 = if i % 8 == 0 {
                // When `i == 0 or 8 or 16`
                self.byte_vec[i_byte_vec]
            } else if i_byte_vec < self.byte_vec.len() - 1 {
                // When  `i == [1, 7] or [9, 15]` and `i_byte_vec == 0 or 1`
                let (part1, part2) = (self.byte_vec[i_byte_vec], self.byte_vec[i_byte_vec + 1]);
                match i % 8 {
                    1 => ((part1 & 0b0111_1111) << 1) | (part2 & 0b0000_0001),
                    2 => ((part1 & 0b0011_1111) << 2) | (part2 & 0b0000_0011),
                    3 => ((part1 & 0b0001_1111) << 3) | (part2 & 0b0000_0111),
                    4 => ((part1 & 0b0000_1111) << 4) | (part2 & 0b0000_1111),
                    5 => ((part1 & 0b0000_0111) << 5) | (part2 & 0b0001_1111),
                    6 => ((part1 & 0b0000_0011) << 6) | (part2 & 0b0011_1111),
                    7 => ((part1 & 0b0000_0001) << 7) | (part2 & 0b0111_1111),
                    _ => panic!("never happen"),
                }
            } else {
                let byte = self.byte_vec[i_byte_vec];
                match (i + size) % 8{
                    1 => byte & 0b1000_0000,
                    2 => byte & 0b1100_0000,
                    3 => byte & 0b1110_0000,
                    4 => byte & 0b1111_0000,
                    5 => byte & 0b1111_1000,
                    6 => byte & 0b1111_1100,
                    7 => byte & 0b1111_1110,
                    0 => byte & 0b1111_1111,
                    _ => panic!("never happen"),
                }
            };
            sub_byte_vec.push(sub_byte);
        }

        let last_byte_len_or_0 = ((i + size) % 8) as u8;
        let last_byte_len = if last_byte_len_or_0 == 0 { 8 } else { last_byte_len_or_0 };

        RawBitVector {
            byte_vec: sub_byte_vec,
            last_byte_len,
        }
    }

    fn validate_index(&self, i: u64) {
        if i >= self.length() { panic!("`i` must be smaller than {} (length of RawBitVector)", self.length()) };
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
