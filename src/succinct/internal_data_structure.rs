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
