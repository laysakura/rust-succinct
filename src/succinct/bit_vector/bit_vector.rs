use super::BitVector;

impl BitVector {
    pub fn access(&self, i: usize) -> bool { self.rbv.access(i) }

    pub fn rank(&self, i: usize) -> usize {
        // TODO O(1) impl
        (0.. (i + 1)).fold(0, |sum, j|
            sum + if self.access(j) { 1 } else { 0 }
        )
    }
}

#[cfg(test)]
mod access_success_tests {
    // well-tested in bit_vector_builder::{builder_from_length_success_tests, builder_from_str_success_tests}
}

#[cfg(test)]
mod access_failure_tests {
    use super::super::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn over_upper_bound() {
        let bv = BitVectorBuilder::from_length(2).build();
        let _ = bv.access(2);
    }
}

#[cfg(test)]
mod rank_success_tests {
    use super::super::{BitVectorBuilder, BitVectorString};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_bv_str, in_i, expected_rank) = $value;
                assert_eq!(
                    BitVectorBuilder::from_str(BitVectorString { s: String::from(in_bv_str) })
                        .build().rank(in_i),
                    expected_rank);
            }
        )*
        }
    }

    parameterized_tests! {
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
    use super::super::BitVectorBuilder;

    #[test]
    #[should_panic]
    fn rank_over_upper_bound() {
        let bv = BitVectorBuilder::from_length(2).build();
        let _ = bv.rank(2);
    }
}
