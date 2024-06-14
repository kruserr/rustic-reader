#!/usr/bin/env bash

set -Eeuo pipefail

cargo publish -p cli-justify
cargo publish -p cli-pdf-to-text
cargo publish -p cli-text-reader
cargo publish -p rustic-reader
