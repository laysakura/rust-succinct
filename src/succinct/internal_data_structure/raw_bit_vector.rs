use super::bit_vector_string::BitVectorString;

pub struct RawBitVector {
    byte_vec: Vec<u8>,
    last_byte_len: usize,  // TODO better to be u8
}

impl RawBitVector {
    pub fn from_length(length: usize) -> RawBitVector {
        if length == 0 { panic!("length must be > 0.") };

        let last_byte_len_or_0 = length % 8;
        RawBitVector {
            byte_vec: vec![0; length / 8 + 1],
            last_byte_len: if last_byte_len_or_0 == 0 { 8 } else { last_byte_len_or_0 },
        }
    }

    pub fn from_str(bit_vector_str: &str) -> RawBitVector {
        let bit_vector_str = BitVectorString::new(bit_vector_str);

        let mut rbv = RawBitVector::from_length(bit_vector_str.s.len());
        for (i, c) in bit_vector_str.s.chars().enumerate() {
            if c == '1' { rbv.set_bit(i); };
        }
        rbv
    }

    pub fn access(&self, i: usize) -> bool {
        self.validate_index(i);
        let byte = self.byte_vec[i / 8];
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

    pub fn set_bit(&mut self, i: usize) {
        self.validate_index(i);
        let byte = self.byte_vec[i / 8];
        self.byte_vec[i / 8] = match i % 8 {
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

    fn bit_length(&self) -> usize {
        (self.byte_vec.len() - 1) * 8 + (self.last_byte_len as usize)
    }

    fn validate_index(&self, i: usize) {
        if i >= self.bit_length() { panic!("`i` must be smaller than {} (length of RawBitVector)", self.bit_length()) };
    }
}

#[cfg(test)]
mod from_length_success_tests {
    use super::RawBitVector;

    struct IndexBitPair(usize, bool);

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
    use super::RawBitVector;

    struct IndexBitPair(usize, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, index_bit_pairs) = $value;
                let rbv = RawBitVector::from_str(in_s);
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
    use super::RawBitVector;

    #[test]
    #[should_panic]
    fn empty() {
        let _ = RawBitVector::from_str("");
    }

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
    use super::RawBitVector;

    struct IndexBitPair(usize, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, bits_to_set, index_bit_pairs) = $value;
                let mut rbv = RawBitVector::from_str(in_s);

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
