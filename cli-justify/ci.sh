#!/usr/bin/env bash

set -Eeuo pipefail

cargo +nightly fmt --all
cargo clippy --all-targets --all-features -- -Dwarnings
cargo test
