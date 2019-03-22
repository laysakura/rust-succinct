extern crate rust_succinct;

use rust_succinct::succinct::bit_vector::{BitVectorBuilder, BitVectorString};

#[test]
fn build_from_length() {
    let bv = BitVectorBuilder::from_length(2).build();
    assert_eq!(bv.access(0), false);
    assert_eq!(bv.access(1), false);
}

#[test]
fn build_from_length_and_set_bit() {
    let bv = BitVectorBuilder::from_length(2)
        .set_bit(0)
        .set_bit(1)
        .set_bit(0)
        .build();
    assert_eq!(bv.access(0), true);
    assert_eq!(bv.access(1), true);
}

#[test]
fn build_from_str() {
    let bv = BitVectorBuilder::from_str(BitVectorString::new("01")).build();
    assert_eq!(bv.access(0), false);
    assert_eq!(bv.access(1), true);
}

#[test]
fn build_from_str_and_set_bit() {
    let bv = BitVectorBuilder::from_str(BitVectorString::new("00"))
        .set_bit(0)
        .set_bit(1)
        .set_bit(0)
        .build();
    assert_eq!(bv.access(0), true);
    assert_eq!(bv.access(1), true);
}
