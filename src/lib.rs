pub mod succinct {
    pub struct BitVector {
        // internal representation
    }

    impl BitVector {
        pub fn access(&self, i: u64) -> bool {}

        pub fn rank(&self, i: u64) -> u64 {}
    }

    pub struct BitVectorBuilder {
        // length w/ set bits OR str representation
    }

    impl BitVectorBuilder {
        pub fn from_length(length: u64) -> BitVectorBuilder {}
        pub fn from_str_repr(bit_vector_str: &str) -> BitVectorBuilder {
            // TODO BitVectorString に変換して処理
        }

        pub fn set_bit(&self, i: u64) -> BitVectorBuilder {}

        pub fn build(&self) -> BitVector {}
    }
}

#[cfg(test)]
mod build_and_access_success_tests {
    use super::succinct::BitVectorBuilder;

    #[test]
    fn build_works() {
        let bv = BitVectorBuilder::from_length(2).build();
        assert_eq!(bv.access(0), false);
        assert_eq!(bv.access(1), false);
    }

    #[test]
    fn build_by_set_bit() {
        let bv = BitVectorBuilder::from_length(2)
            .set_bit(1)
            .build();
        assert_eq!(bv.access(0), false);
        assert_eq!(bv.access(1), true);
    }

    #[test]
    fn build_by_str() {
        let bv = BitVectorBuilder::from_str_repr("101").build();
        assert_eq!(bv.access(0), true);
        assert_eq!(bv.access(1), false);
        assert_eq!(bv.access(2), true);
    }

    #[test]
    fn build_by_str_with_set_bit() {
        let bv = BitVectorBuilder::from_str_repr("101")
            .set_bit(0)
            .set_bit(1)
            .build();
        assert_eq!(bv.access(0), true);
        assert_eq!(bv.access(1), true);
        assert_eq!(bv.access(2), true);
    }
}

#[cfg(test)]
mod build_and_access_failure_tests {
    use super::succinct::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn access_over_upper_bound_causes_panic() {
        let bv = BitVectorBuilder::from_length(2).build();
        let _ = bv.access(2);
    }
}

#[cfg(test)]
mod rank_success_tests {
    use super::succinct::BitVectorBuilder;

    macro_rules! parameterized_rank_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_bv_str, in_i, expected_rank) = $value;
                assert_eq!(BitVectorBuilder::from_str_repr(in_bv_str).build().rank(in_i), expected_rank);
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
        rank6_3: ("10010", 0, 1),
        rank6_4: ("10010", 1, 2),
        rank6_5: ("10010", 0, 2),
    }
}

#[cfg(test)]
mod rank_failure_tests {
    use super::succinct::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn rank_over_upper_bound_causes_panic() {
        let bv = BitVectorBuilder::from_length(2).build();
        let _ = bv.rank(2);
    }
}
