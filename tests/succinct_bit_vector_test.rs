extern crate succinct;

use succinct::{BitVectorBuilder, BitVectorString};

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

#[test]
fn fuzzing_test() {
    let samples = 10000;

    fn access_from_str(s: &str, i: u64) -> bool {
        s.chars().collect::<Vec<char>>()[i as usize] == '1'
    }

    fn rank_from_str(s: &str, i: u64) -> u64 {
        let chs = s.chars().collect::<Vec<char>>();
        let mut rank: u64 = 0;
        for j in 0..=i as usize {
            if chs[j] == '1' {
                rank += 1
            };
        }
        rank
    }

    fn select_from_str(s: &str, num: u64) -> Option<u64> {
        if num == 0 {
            return Some(0);
        }

        let mut cnt: u64 = 0;
        for (i, ch) in s.chars().enumerate() {
            if ch == '1' {
                cnt += 1;
            }
            if cnt == num {
                return Some(i as u64);
            }
        }
        None
    }

    for _ in 0..samples {
        let s = &format!("{:b}", rand::random::<u128>());
        eprintln!("build(): bit vec = \"{}\"", s);

        let bvs = BitVectorString::new(s);
        let bv = BitVectorBuilder::from_str(bvs).build();

        for i in 0..s.len() {
            eprintln!("access(): bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                bv.access(i as u64),
                access_from_str(s, i as u64),
                "bit vec = \"{}\", i={}, BitVector::access()={}, access_from_str={}",
                s,
                i,
                bv.access(i as u64),
                access_from_str(s, i as u64)
            );

            eprintln!("rank(): bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                bv.rank(i as u64),
                rank_from_str(s, i as u64),
                "bit vec = \"{}\", i={}, BitVector::rank()={}, rank_from_str={}",
                s,
                i,
                bv.rank(i as u64),
                rank_from_str(s, i as u64)
            );

            let num = i as u64;
            eprintln!("select(): bit vec = \"{}\", num = {}, ", s, num);
            assert_eq!(
                bv.select(num),
                select_from_str(s, num),
                "bit vec = \"{}\", num={}, BitVector::select()={:?}, select_from_str={:?}",
                s,
                num,
                bv.select(num),
                select_from_str(s, num)
            );
        }
    }
}
