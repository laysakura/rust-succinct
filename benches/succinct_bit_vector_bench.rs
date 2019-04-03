#[macro_use]
extern crate criterion;

use criterion::{BatchSize, Criterion};
use std::time::Duration;
use succinct::{BitVectorBuilder, BitVectorString};

const NS: [u64; 5] = [1 << 16, 1 << 17, 1 << 18, 1 << 19, 1 << 20];

fn c() -> Criterion {
    Criterion::default()
        .sample_size(2)
        .warm_up_time(Duration::from_secs(1))
}

fn builder_from_length_benchmark(_: &mut Criterion) {
    c().bench_function_over_inputs(
        "BitVectorBuilder::from_length(N).build()",
        |b, &&n| b.iter(|| BitVectorBuilder::from_length(n).build()),
        &NS,
    );
}

fn builder_from_str_benchmark(_: &mut Criterion) {
    c().bench_function_over_inputs(
        "BitVectorBuilder::from_str(\"00...(repeated N-times)\").build()",
        |b, &&n| {
            b.iter_batched(
                || {
                    let s = String::from_utf8(vec!['0' as u8; n as usize]).unwrap();
                    BitVectorString::new(&s)
                },
                |bvs| BitVectorBuilder::from_str(bvs).build(),
                BatchSize::SmallInput,
            )
        },
        &NS,
    );
}

fn rank_benchmark(_: &mut Criterion) {
    c().bench_function_over_inputs(
        "BitVector::rank(N)",
        move |b, &&n| {
            b.iter_batched(
                || BitVectorBuilder::from_length(n).build(),
                |bv| (bv).rank(n - 1),
                BatchSize::SmallInput,
            )
        },
        &NS,
    );
}

criterion_group!(
    benches,
    builder_from_length_benchmark,
    builder_from_str_benchmark,
    rank_benchmark,
);
criterion_main!(benches);
