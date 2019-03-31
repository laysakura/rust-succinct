use super::BitVectorString;

impl super::BitVectorString {
    /// Constructor.
    ///
    /// '0' is interpreted as _0_.
    /// '1' is interpreted as _1_.
    /// '_' is just ignored.
    ///
    /// # Examples
    /// ```
    /// use succinct::bit_vector::BitVectorString;
    ///
    /// let bvs = BitVectorString::new("01");
    /// assert_eq!(bvs.str(), "01");
    ///
    /// let bvs = BitVectorString::new("0111_0101");
    /// assert_eq!(bvs.str(), "01110101");
    /// ```
    ///
    /// # Panics
    /// When:
    /// - `s` contains any character other than '0', '1', and '_'.
    /// - `s` does not contain any '0' or '1'
    pub fn new(s: &str) -> BitVectorString {
        let parsed = s
            .chars()
            .filter(|c| match c {
                '0' => true,
                '1' => true,
                '_' => false,
                _ => panic!("`str` must consist of '0' or '1'. '{}' included.", c),
            })
            .collect::<String>();

        if parsed.is_empty() {
            panic!("`str` must contain any '0' or '1'.")
        }

        BitVectorString {
            s: String::from(parsed),
        }
    }

    /// Getter.
    pub fn str(&self) -> &str {
        &self.s
    }
}

#[cfg(test)]
mod new_success_tests {
    use super::super::BitVectorString;

    macro_rules! parameterized_from_valid_str_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, expected_str) = $value;
                let bvs = BitVectorString::new(in_s);
                assert_eq!(bvs.str(), expected_str);
            }
        )*
        }
    }

    parameterized_from_valid_str_tests! {
        s1: ("0", "0"),
        s2: ("1", "1"),
        s3: ("00", "00"),
        s4: ("01", "01"),
        s5: ("10", "10"),
        s6: ("11", "11"),
        s7_1: ("01010101010111001000001", "01010101010111001000001"),
        s7_2: ("01010101_01011100_1000001", "01010101010111001000001"),
    }
}

#[cfg(test)]
mod new_failure_tests {
    use super::super::BitVectorString;

    macro_rules! parameterized_from_invalid_str_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let in_s = $value;
                let _ = BitVectorString::new(in_s);
            }
        )*
        }
    }

    parameterized_from_invalid_str_tests! {
        s0: "",
        s1: " ",
        s2: " 0",
        s3: "0 ",
        s4: "1 0",
        s5: "０",
        s6: "１",
        s7: "012",
        s8: "01二",
        s9: "_____",
    }
}
