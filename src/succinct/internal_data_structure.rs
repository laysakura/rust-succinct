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
