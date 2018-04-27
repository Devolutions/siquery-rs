#!/bin/bash

set -x
set -e

cargo build
cargo test
[ "$RUST_NIGHTLY" != "1" ] || cargo test --features nightly
cargo bench
cargo doc
