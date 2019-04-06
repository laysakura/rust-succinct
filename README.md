# Succinct.rs

Succinct Data Structures library for Rust.

[Master API Docs](https://laysakura.github.io/succinct.rs/succinct/)
|
[Released API Docs](https://docs.rs/crate/succinct_rs)
|
[Benchmark Results](https://laysakura.github.io/succinct.rs/criterion/report/)
|
[Changelog](https://github.com/laysakura/succinct.rs/blob/master/CHANGELOG.md)

[![Build Status](https://travis-ci.com/laysakura/succinct.rs.svg?branch=master)](https://travis-ci.com/laysakura/succinct.rs)
[![Crates.io](https://img.shields.io/crates/v/succinct_rs.svg)](https://crates.io/crates/succinct_rs)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.33+-lightgray.svg)](https://github.com/laysakura/succinct.rs#rust-version-supports)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/laysakura/succinct.rs/blob/master/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/laysakura/succinct.rs/blob/master/LICENSE-APACHE)

Succinct.rs is a library to provide succinct data structures with _simple API_ and _high performance_.

Currently, **[Succinct Bit Vector](https://laysakura.github.io/succinct.rs/succinct/bit_vector/struct.BitVector.html)** is supported.

## Table of Contents
- [Table of Contents](#table-of-contents)
  - [Quickstart](#quickstart)
  - [Features](#features)
  - [Versions](#versions)
  - [Roadmap](#roadmap)
  - [Contributing](#contributing)
  - [License](#license)

## Quickstart

To use with Succinct.rs, add the following to your `Cargo.toml` file:

```toml
[dependencies]
succinct_rs = "0.1"
```

### [Succinct Bit Vector](https://laysakura.github.io/succinct.rs/succinct/bit_vector/struct.BitVector.html) Usage

```rust
use succinct::bit_vector::{BitVectorBuilder, BitVectorString};

// `01001` built by `from_length()` and `set_bit()`
let bv = BitVectorBuilder::from_length(5)
    .set_bit(1)
    .set_bit(4)
    .build();

assert_eq!(bv.access(0), false);  // [0]1001; 0th bit is '0' (false)
assert_eq!(bv.access(1), true);   // 0[1]001; 1st bit is '1' (true)
assert_eq!(bv.access(4), true);   // 0100[1]; 4th bit is '1' (true)

assert_eq!(bv.rank(0), 0);  // [0]1001; Range [0, 0] has no '1'
assert_eq!(bv.rank(3), 1);  // [0100]1; Range [0, 3] has 1 '1'
assert_eq!(bv.rank(4), 2);  // [01001]; Range [0, 4] has 2 '1's

assert_eq!(bv.select(0), Some(0)); // []01001; Minimum `i` where range [0, i] has 0 '1's is `i=0`
assert_eq!(bv.select(1), Some(1)); // 0[1]001; Minimum `i` where range [0, i] has 1 '1's is `i=1`
assert_eq!(bv.select(2), Some(4)); // 0100[1]; Minimum `i` where range [0, i] has 2 '1's is `i=4`
assert_eq!(bv.select(3), None);    // There is no `i` where range [0, i] has 3 '1's

// `10010` built by `from_str()`
let bv = BitVectorBuilder::from_str(BitVectorString::new("1001_0")).build();  // Tips: BitVectorString::new() ignores '_'.
```

## Features

- **Arbitrary length support with minimum working memory**: Succinct.rs provides virtually _arbitrary length_ of data structures. There are carefully designed to use as small memory space as possible.
- **Simple public APIs**: Each data structures almost only have very basic operations for the data structure. `succinct::BitVector`, for example, has only `access()`, `rank()`, and `select()`.
- **Latest benchmark results are always accessible**: Succinct.rs is continuously benchmarked in Travis CI using [Criterion.rs](https://crates.io/crates/criterion). Graphical benchmark results are published [here](https://laysakura.github.io/succinct.rs/criterion/report/).

### [Succinct Bit Vector](https://laysakura.github.io/succinct.rs/succinct/bit_vector/struct.BitVector.html) Complexity

When the length of a `BitVector` is `N`:

|                  | [build()](https://laysakura.github.io/succinct.rs/succinct/bit_vector/struct.BitVectorBuilder.html#method.build) | [access()](https://laysakura.github.io/succinct.rs/succinct/bit_vector/struct.BitVector.html#method.access) | [rank()](https://laysakura.github.io/succinct.rs/succinct/bit_vector/struct.BitVector.html#method.rank) | [select()](https://laysakura.github.io/succinct.rs/succinct/bit_vector/struct.BitVector.html#method.select) |
|------------------|--------------------------------------------------------|------------|----------|------------|
| Time-complexity  | _O(N)_                                                 | _O(1)_     | _O(1)_   | _O(log N)_ |
| Space-complexity | _N + o(N)_                                             | _0_        | _O(log N)_   | _O(log N)_     |

(Actually, `select()`'s time-complexity can be _O(1)_ with complex implementation but Succinct.rs, like many other libraries, uses binary search of `rank()`'s result).

## Versions
Succinct.rs uses [semantic versioning](http://semver.org/spec/v2.0.0.html).

Since current major version is _0_, minor version update might involve breaking public API change (although it is carefully avoided).

### Rust Version Supports

Succinct.rs is continuously tested with these Rust versions in Travis CI:

- 1.33.0
- Latest stable version
- Beta version
- Nightly build

So it expectedly works with Rust 1.33.0 and any newer versions.

Older versions may also work, but are not tested or guaranteed.

## Roadmap

Succinct.rs has plan to provide these succinct data structures.

1. Succinct Bit Vector **(done)**
2. [LOUDS](https://dl.acm.org/citation.cfm?id=1398646)
3. [SuRF](http://www.pdl.cmu.edu/PDL-FTP/Storage/surf_sigmod18.pdf)

## Contributing

Any kind of pull requests are appreciated.

Currently, there are not many rules for contribution.
But at least your pull requests must pass Travis CI.

## License

Succinct.rs is dual licensed under the Apache 2.0 license and the MIT license.
