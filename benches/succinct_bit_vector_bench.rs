#[macro_use]
extern crate criterion;

use criterion::{BatchSize, Criterion};
use std::time::Duration;
use succinct_rs::{BitVectorBuilder, BitVectorString};

const NS: [u64; 5] = [1 << 16, 1 << 17, 1 << 18, 1 << 19, 1 << 20];

fn c() -> Criterion {
    Criterion::default()
        .sample_size(10) // must be >= 10 for Criterion v0.3
        .warm_up_time(Duration::from_secs(1))
        .with_plots()
}

fn git_hash() -> String {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}

fn builder_from_length_benchmark(_: &mut Criterion) {
    c().bench_function_over_inputs(
        &format!("[{}] BitVectorBuilder::from_length(N).build()", git_hash()),
        |b, &&n| b.iter(|| BitVectorBuilder::from_length(n).build()),
        &NS,
    );
}

fn builder_from_str_benchmark(_: &mut Criterion) {
    c().bench_function_over_inputs(
        &format!(
            "[{}] BitVectorBuilder::from_str(\"00...(repeated N-times)\").build()",
            git_hash()
        ),
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
    let times = 1_000_000;

    c().bench_function_over_inputs(
        &format!("[{}] BitVector::rank(N) {} times", git_hash(), times),
        move |b, &&n| {
            b.iter_batched(
                || BitVectorBuilder::from_length(n).build(),
                |bv| {
                    // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                    // rank() takes too short compared to build(). So loop many times.
                    for _ in 0..times {
                        assert_eq!(bv.rank(n - 1), 0);
                    }
                },
                BatchSize::SmallInput,
            )
        },
        &NS,
    );
}

fn select_benchmark(_: &mut Criterion) {
    let times = 1_000;

    c().bench_function_over_inputs(
        &format!("[{}] BitVector::select(N) {} times", git_hash(), times),
        move |b, &&n| {
            b.iter_batched(
                || {
                    let mut builder = BitVectorBuilder::from_length(n);
                    for i in 0..n {
                        builder.set_bit(i);
                    }
                    builder.build()
                },
                |bv| {
                    // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                    // rank() takes too short compared to build(). So loop many times.
                    for _ in 0..times {
                        assert_eq!(bv.select(n - 1), Some(n - 2));
                    }
                },
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
    select_benchmark,
);
criterion_main!(benches);
