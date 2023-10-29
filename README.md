# rmg

> Rust: Tiny And Fast Manga/Image Viewer

## Demo

![](./assets/demo.jpg)

## Install

+ Dependency
  + Linux: x11 / wayland
  + Windows:
  + Mac:
+ Optional Dependency
  + libheif

```bash
# github
see: https://github.com/rsuu/rmg/releases

# cargo
cargo install rmg

# git
git clone --depth 1 https://github.com/rsuu/rmg
cd rmg
cargo build --release
```

## Usage

```bash
rmg --help

rmg tests/bit/png

rmg file.tar

rmg file.gif

rmg --size 600x600 file.tar

rmg --size 100x600 --config ./tests/other/config.rs file.tar
```

## KeyMap

|Key| |
|:-|:-|
k/Up    | move up
j/Down  | move down
h/Left  | move left
r/Right | move right
q       | quit

## Configuration

> config: https://raw.githubusercontent.com/rsuu/rmg/main/tests/other/config.rs

NOTE: You must create the file first.

+ Config file path
  + Linux: `$HOME/.config/rmg/config.rs`
  + Mac: `$HOME/Library/Application Support/rmg/config.rs`
  + Windows: `C:\Users\<USER>\AppData\<USER>\rmg\config.rs`

## Supported Formats

| Format | Supported | Default | Dependency | Mode
|:-|:-|:-|:-|:-|
.jpg          | ✔     | ✔ |         | Scroll/Once
.png          | ✔     | ✔ |         | Scroll/Once
.heic / .avif | ✔     | ✖ | libheif | Scroll/Once
.gif          | ✔     | ✔ |         | Once
.aseprite     | ✔     | ✖ |         | Once
.svg          | (dev) | ✖ |         | Scroll/Once

| Format | Supported | Default | Dependency
|:-|:-|:-|:-|
directory   | ✔ | ✔ |
.tar        | ✔ | ✔ |
.zip / .cbz | ✔ | ✔ |
.rar        | ✖ |   |
.zst        | ✖ |   |

## Features

```bash
# HEIF/HEIC
cargo run --release -F "de_heic"

# SVG and Aseprite
cargo run --release -F "de_svg" -F "de_aseprite"

# see [CpuExtensions](https://docs.rs/fast_image_resize/latest/fast_image_resize/index.html#resize-rgb8-image-u8x3-4928x3279--852x567)
cargo run --release -F "avx2"
    # AVX2
cargo run --release -F "sse4_1"
    # SSE4_1

cargo run --release -F "full"
```
