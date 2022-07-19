set "RUSTFLAGS=-Awarnings"
set "RUSTFMT_CI=1"

:: Print version information
rustc -Vv || exit /b 1
cargo -V || exit /b 1

:: Build and test main crate
cargo build || exit /b 1
cargo test || exit /b 1
