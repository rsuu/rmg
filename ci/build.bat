set "RUSTFLAGS=-Awarnings"
set "RUSTFMT_CI=1"

:: Print version information
rustc -Vv || exit /b 1
cargo -V || exit /b 1


Invoke-WebRequest -Uri "http://www.libsdl.org/release/SDL2-devel-2.0.22-mingw.tar.gz" -OutFile "sdl2.tar.gz"
tar -xvf "sdl2.tar.gz"

Get-ChildItem -Force
Get-Location

Copy-Item -Path "SDL2-2.0.22\x86_64-w64-mingw32\lib\*" -Destination "C:\Users\runneradmin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\" -Recurse
Copy-Item -Path "SDL2-2.0.22\x86_64-w64-mingw32\bin\*" -Destination "C:\Users\runneradmin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib\" -Recurse
Copy-Item -Path "SDL2-2.0.22\x86_64-w64-mingw32\bin\*" -Destination "D:\a\rmg\rmg" -Recurse

:: Build and test main crate
cargo build || exit /b 1
cargo test || exit /b 1
