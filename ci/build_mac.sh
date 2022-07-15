#!/bin/bash

set -euo pipefail

export RUSTFLAGS="-Awarnings"
export RUSTFMT_CI=1

# Print version information
rustc -Vv
cargo -V

brew install SDL2
brew install SDL2_image
brew install SDL2_ttf

# Build and test main crate
cargo build
cargo test
