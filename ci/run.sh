#!/bin/sh

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

# Tests are all super fast anyway, and they fault often enough on travis that
# having only one thread increases debuggability to be worth it.
export RUST_TEST_THREADS=1
#export RUST_BACKTRACE=full
#export RUST_TEST_NOCAPTURE=1


echo "RUSTFLAGS=${RUSTFLAGS}"
echo "FEATURES=${FEATURES}"
echo "NORUN=${NORUN}"
echo "CARGO_SUBCMD=${CARGO_SUBCMD}"

cargo test --target=${TARGET} $1
cargo test --release --target=${TARGET} $1
