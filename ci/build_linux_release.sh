#!/bin/bash

set -euo pipefail

export RUSTFLAGS="-Awarnings"
export RUSTFMT_CI=1

# Print version information
rustc -Vv
cargo -V

sudo apt-get -y install libsdl2-dev libsdl2-image-2.0-0 libsdl2-ttf-2.0-0 libsdl2-image-dev libsdl2-ttf-dev

# Build and test main crate
cargo build --release
