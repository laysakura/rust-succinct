use super::{Louds, LoudsBuilder};
use crate::bit_vector::BitVectorBuilder;
use crate::BitString;

impl super::LoudsBuilder {
    /// O(log N)
    pub fn from_bit_string(bs: BitString) -> LoudsBuilder {
        LoudsBuilder::validate_lbs(&bs);
        let bv_builder = BitVectorBuilder::from_bit_string(bs);
        LoudsBuilder { bv_builder }
    }

    /// O(log N)
    pub fn build(&self) -> Louds {
        let bv = self.bv_builder.build();
        // TODO How should we pass built bit string to validate_lbs() ?
        Louds { lbs: bv }
    }

    /// Checks if `bs` satisfy the LBS's Necessary and sufficient condition:
    ///
    /// - Starts from "10"
    /// - In the range of _[0, i]_ for any _i (< length of LBS)_;
    ///     - _<u>the number of '0'</u> <= <u>the number of '1'</u> + 1_, because:
    ///         - Each node, including virtual root (node num = 0), has one '0'.
    ///         - Each node is derived from one '1'.
    /// - In the range of _[0, <u>length of LBS</u>)_;
    ///     - _<u>the number of '0'</u> == <u>the number of '1'</u> + 1_
    fn validate_lbs(bs: &BitString) {
        let s = bs.str();

        assert!(s.starts_with("10"));

        let (mut cnt0, mut cnt1) = (0u64, 0u64);
        for (i, ch) in s.chars().enumerate() {
            match ch {
                '0' => cnt0 += 1,
                '1' => cnt1 += 1,
                c => panic!("LBS contains invalid character '{}'", c),
            }
            assert!(
                cnt0 <= cnt1 + 1,
                "At index {}, the number of '0' ({}) == (the number of '1' ({})) + 2.",
                i,
                cnt0,
                cnt1,
            );
        }

        assert_eq!(cnt0, cnt1 + 1);
    }
}

#[cfg(test)]
mod validate_lbs_success_tests {
    use crate::{BitString, LoudsBuilder};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let s = $value;
                let bs = BitString::new(s);
                LoudsBuilder::validate_lbs(&bs);
            }
        )*
        }
    }

    parameterized_tests! {
        t1: "10_0",
        t2: "10_10_0",
        t3: "10_1110_10_0_1110_0_0_10_110_0_0_0",
        t4: "10_11111111110_0_0_0_0_0_0_0_0_0_0",
    }
}

#[cfg(test)]
mod validate_lbs_failure_tests {
    use crate::{BitString, LoudsBuilder};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let s = $value;
                let bs = BitString::new(s);
                LoudsBuilder::validate_lbs(&bs);
            }
        )*
        }
    }

    parameterized_tests! {
        t1: "0",
        t2: "1",
        t3: "00",
        t4: "01",
        t5: "10",
        t6: "11",
        t7: "00_0",
        t8: "01_0",
        t9: "11_0",
        t10: "10_1",
        t11: "10_10",
        t12: "10_01",
        t13: "10_1110_10_0_1110_0_0_10_110_0_0_1",
    }
}
