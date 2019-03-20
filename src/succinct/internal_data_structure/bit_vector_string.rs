pub struct BitVectorString { pub s: String }

impl BitVectorString {
    pub fn new(s: &str) -> BitVectorString {
        // TODO split into procedure like `assert_valid_str`
        // should not be empty
        if s.is_empty() { panic!("`str` must not be empty.") }

        // should contain only '0' or '1'
        for c in s.chars() {
            match c {
                '0' => (),
                '1' => (),
                _ => panic!("`str` must consist of '0' or '1'. '{}' included.", c),
            }
        }

        BitVectorString { s: String::from(s) }
    }
}

#[cfg(test)]
mod new_success_tests {
    use super::BitVectorString;

    macro_rules! parameterized_from_valid_str_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let in_s = $value;
                let _ = BitVectorString::new(in_s);
            }
        )*
        }
    }

    parameterized_from_valid_str_tests! {
        s1: "0",
        s2: "1",
        s3: "00",
        s4: "01",
        s5: "10",
        s6: "11",
        s7: "01010101010111001000001",
    }
}

#[cfg(test)]
mod new_failure_tests {
    use super::BitVectorString;

    #[test]
    #[should_panic]
    fn from_empty_str() {
        let _ = BitVectorString::new("");
    }

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
        s1: " ",
        s2: " 0",
        s3: "0 ",
        s4: "1 0",
        s5: "０",
        s6: "１",
        s7: "012",
        s8: "01二",
    }
}
