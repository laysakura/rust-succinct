#[cfg(test)]
mod tests {
    #[test]
    fn build_works() {
        let bv = succinct::BitVector::new(2);  // takes length
        bv.build();
        assert_eq!(bv.access(0), false);
        assert_eq!(bv.access(1), false);
    }

    #[test]
    fn without_build_works() {
        let bv = succinct::BitVector::new(2);  // takes length
        assert_eq!(bv.access(0), false);  // build() internally
        assert_eq!(bv.access(1), false);
    }

    #[test]
    #[should_panic]
    fn access_over_lower_bound_causes_panic() {
        let bv = succinct::BitVector::new(2).build();
        let _ = bv.access(-1);
    }

    #[test]
    #[should_panic]
    fn access_over_upper_bound_causes_panic() {
        let bv = succinct::BitVector::new(2).build();
        let _ = bv.access(2);
    }

    #[test]
    fn build_by_set_bit() {
        let bv = succinct::BitVector::new(2)
            .set_bit(1)
            .build();
        assert_eq!(bv.access(0), false);
        assert_eq!(bv.access(1), true);
    }

    #[test]
    fn build_by_str() {
        let bv = succinct::BitVector::new("101").build();
        assert_eq!(bv.access(0), true);
        assert_eq!(bv.access(1), false);
        assert_eq!(bv.access(2), true);
    }

    #[test]
    fn build_by_str_with_set_bit() {
        let bv = succinct::BitVector::new("101")
            .set_bit(0)
            .set_bit(1)
            .build();
        assert_eq!(bv.access(0), true);
        assert_eq!(bv.access(1), true);
        assert_eq!(bv.access(2), true);
    }
}
