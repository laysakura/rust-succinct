# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.4.0]

### Added
- `succinct_rs::{Louds, LoudsBuilder, LoudsIndex, LoudsNodeNum}`

## [v0.3.0]

### Added
- `succinct_rs::BitVector::rank0()`
- `succinct_rs::BitVector::select0()`

## [v0.2.0]

### Added
- `succinct_rs::BitString`
- `succinct_rs::BitVectorBuilder::from_bit_string()`

### Removed
- `succinct_rs::BitVectorString`
- `succinct_rs::BitVectorBuilder::from_str()`

## [v0.1.1]

### Fixed
- Adds `readme = "README.md"` in `Cargo.toml` in order to display README contents in crates.io.

## [v0.1.0]

### Added
- `succinct_rs::BitVector` and its builders: `succinct_rs::BitVectorBuilder` and `succinct_rs::BitVectorString`.

[Unreleased]: https://github.com/laysakura/succinct.rs/compare/v0.3.0...HEAD
[v0.3.0]: https://github.com/laysakura/succinct.rs/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/laysakura/succinct.rs/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/laysakura/succinct.rs/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/laysakura/succinct.rs/compare/3d425b4...v0.1.0
