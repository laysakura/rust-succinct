use std::collections::HashSet;
use super::internal_data_structure::raw_bit_vector::RawBitVector;

pub struct BitVector {
    rbv: RawBitVector,
}

impl BitVector {
    pub fn access(&self, i: usize) -> bool { self.rbv.access(i) }

    pub fn rank(&self, i: usize) -> usize {
        // TODO O(1) impl
        (0.. (i + 1)).fold(0, |sum, j|
            sum + if self.access(j) { 1 } else { 0 }
        )
    }
}

enum BitVectorSeed {
    Length(usize),
    Str(String),
}

pub struct BitVectorBuilder {
    seed: BitVectorSeed,
    bits_set: HashSet<usize>,
}

impl BitVectorBuilder {
    pub fn from_length(length: usize) -> BitVectorBuilder {
        BitVectorBuilder { seed: BitVectorSeed::Length(length), bits_set: HashSet::new() }
    }

    pub fn from_str(bit_vector_str: &str) -> BitVectorBuilder {
        BitVectorBuilder { seed: BitVectorSeed::Str(String::from(bit_vector_str)), bits_set: HashSet::new() }
    }

    // TODO copy every time when set_bit() called?
    pub fn set_bit(mut self, i: usize) -> BitVectorBuilder {
        self.bits_set.insert(i);
        self
    }

    pub fn build(&self) -> BitVector {
        let mut rbv = match &self.seed {
            BitVectorSeed::Length(n) => RawBitVector::from_length(*n),
            BitVectorSeed::Str(s) => RawBitVector::from_str(s),
        };
        for bit in &self.bits_set { rbv.set_bit(*bit) }
        BitVector { rbv }
    }
}


#[cfg(test)]
mod build_and_access_success_tests {
    use super::BitVectorBuilder;

    #[test]
    fn build() {
        let bv = BitVectorBuilder::from_length(2).build();
        assert_eq!(bv.access(0), false);
        assert_eq!(bv.access(1), false);
    }

    #[test]
    fn build_from_set_bit() {
        let bv = BitVectorBuilder::from_length(2)
            .set_bit(1)
            .build();
        assert_eq!(bv.access(0), false);
        assert_eq!(bv.access(1), true);
    }

    #[test]
    fn build_from_str() {
        let bv = BitVectorBuilder::from_str("101").build();
        assert_eq!(bv.access(0), true);
        assert_eq!(bv.access(1), false);
        assert_eq!(bv.access(2), true);
    }

    #[test]
    fn build_from_str_with_set_bit() {
        let bv = BitVectorBuilder::from_str("101")
            .set_bit(0)
            .set_bit(1)
            .build();
        assert_eq!(bv.access(0), true);
        assert_eq!(bv.access(1), true);
        assert_eq!(bv.access(2), true);
    }

    #[test]
    fn build_from_set_bit_on_same_bit_twice() {
        let bv = BitVectorBuilder::from_length(2)
            .set_bit(1)
            .set_bit(1)
            .build();
        assert_eq!(bv.access(0), false);
        assert_eq!(bv.access(1), true);
    }
}

#[cfg(test)]
mod build_and_access_failure_tests {
    use super::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn build_empty_from_length() {
        let _ = BitVectorBuilder::from_length(0).build();
    }

    #[test]
    #[should_panic]
    fn build_empty_from_str() {
        let _ = BitVectorBuilder::from_str("").build();
    }

    #[test]
    #[should_panic]
    fn set_bit_over_upper_bound() {
        let _ = BitVectorBuilder::from_length(2)
            .set_bit(2)
            .build();
    }

    #[test]
    #[should_panic]
    fn access_over_upper_bound() {
        let bv = BitVectorBuilder::from_length(2).build();
        let _ = bv.access(2);
    }
}

#[cfg(test)]
mod rank_success_tests {
    use super::BitVectorBuilder;

    macro_rules! parameterized_rank_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_bv_str, in_i, expected_rank) = $value;
                assert_eq!(BitVectorBuilder::from_str(in_bv_str).build().rank(in_i), expected_rank);
            }
        )*
        }
    }

    parameterized_rank_tests! {
        rank1_1: ("0", 0, 0),

        rank2_1: ("00", 0, 0),
        rank2_2: ("00", 1, 0),

        rank3_1: ("01", 0, 0),
        rank3_2: ("01", 1, 1),

        rank4_1: ("10", 0, 1),
        rank4_2: ("10", 1, 1),

        rank5_1: ("11", 0, 1),
        rank5_2: ("11", 1, 2),

        rank6_1: ("10010", 0, 1),
        rank6_2: ("10010", 1, 1),
        rank6_3: ("10010", 2, 1),
        rank6_4: ("10010", 3, 2),
        rank6_5: ("10010", 4, 2),
    }
}

#[cfg(test)]
mod rank_failure_tests {
    use super::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn rank_over_upper_bound() {
        let bv = BitVectorBuilder::from_length(2).build();
        let _ = bv.rank(2);
    }
}
