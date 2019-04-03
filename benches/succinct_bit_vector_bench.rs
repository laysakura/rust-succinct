#[macro_use]
extern crate criterion;

use criterion::Criterion;
use succinct::{BitVectorBuilder, BitVectorString};

const NS: [u64; 5] = [1 << 16, 1 << 17, 1 << 18, 1 << 19, 1 << 20];

fn builder_from_length_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "BitVectorBuilder::from_length(N).build()",
        |b, &&n| b.iter(|| BitVectorBuilder::from_length(n).build()),
        &NS,
    );
}

fn builder_from_str_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "BitVectorBuilder::from_str(\"00...(repeated N-times)\").build()",
        |b, &&n| {
            b.iter(|| {
                let s = String::from_utf8(vec!['0' as u8; n as usize]).unwrap();
                let bvs = BitVectorString::new(&s);
                BitVectorBuilder::from_str(bvs).build()
            })
        },
        &NS,
    );
}

criterion_group!(
    benches,
    builder_from_length_benchmark,
    builder_from_str_benchmark
);
criterion_main!(benches);
