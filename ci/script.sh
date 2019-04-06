#!/bin/sh
set -eux

SCRIPT_DIR=$(cd $(dirname $0); pwd)
. $SCRIPT_DIR/functions.sh

cargo build --release --verbose --all
cargo test --release --verbose --all
cargo fmt --all -- --check
cargo doc
find target/doc/
cargo bench --all && rm -rf target/doc/criterion && mv -f target/criterion target/doc/
