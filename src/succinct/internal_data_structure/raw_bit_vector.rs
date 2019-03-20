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
mod access_success_tests {
    use super::RawBitVector;

    #[test]
    fn build() {
        let rbv = RawBitVector::from_length(2);
        assert_eq!(rbv.access(0), false);
        assert_eq!(rbv.access(1), false);
    }

    #[test]
    fn build_from_set_bit() {
        let mut rbv = RawBitVector::from_length(2);
        rbv.set_bit(1);
        assert_eq!(rbv.access(0), false);
        assert_eq!(rbv.access(1), true);
    }

    #[test]
    fn build_from_str() {
        let rbv = RawBitVector::from_str("101");
        assert_eq!(rbv.access(0), true);
        assert_eq!(rbv.access(1), false);
        assert_eq!(rbv.access(2), true);
    }

    #[test]
    fn build_from_str_with_set_bit() {
        let mut rbv = RawBitVector::from_str("101");
        rbv.set_bit(0);
        rbv.set_bit(1);
        assert_eq!(rbv.access(0), true);
        assert_eq!(rbv.access(1), true);
        assert_eq!(rbv.access(2), true);
    }

    #[test]
    fn build_from_set_bit_on_same_bit_twice() {
        let mut rbv = RawBitVector::from_length(2);
        rbv.set_bit(1);
        rbv.set_bit(1);
        assert_eq!(rbv.access(0), false);
        assert_eq!(rbv.access(1), true);
    }
}

#[cfg(test)]
mod access_failure_tests {
    use super::RawBitVector;

    #[test]
    #[should_panic]
    fn build_empty_from_length() {
        let _ = RawBitVector::from_length(0);
    }

    #[test]
    #[should_panic]
    fn build_empty_from_str() {
        let _ = RawBitVector::from_str("");
    }

    #[test]
    #[should_panic]
    fn set_bit_over_upper_bound() {
        let mut rbv = RawBitVector::from_length(2);
        rbv.set_bit(2);
    }

    #[test]
    #[should_panic]
    fn access_over_upper_bound() {
        let rbv = RawBitVector::from_length(2);
        let _ = rbv.access(2);
    }
}
