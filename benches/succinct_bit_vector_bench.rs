#[macro_use]
extern crate criterion;

use criterion::Criterion;
use succinct::{BitVectorBuilder, BitVectorString};

fn builder_from_length_benchmark(c: &mut Criterion) {
    c.bench_function("BitVectorBuilder::from_length(2^20).build()", |b| {
        b.iter(|| BitVectorBuilder::from_length(1 << 20).build())
    });
}

fn builder_from_str_benchmark(c: &mut Criterion) {
    c.bench_function(
        "BitVectorBuilder::from_str(\"00...\" (2^20 \"0\"s)).build()",
        |b| {
            b.iter(|| {
                let s = String::from_utf8(vec!['0' as u8; 1 << 20]).unwrap();
                let bvs = BitVectorString::new(&s);
                BitVectorBuilder::from_str(bvs).build()
            })
        },
    );
}

criterion_group!(
    benches,
    builder_from_length_benchmark,
    builder_from_str_benchmark
);
criterion_main!(benches);
