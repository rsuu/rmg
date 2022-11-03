# rmg

> Rust: Tiny Manga Reader

## Demo

![](./assets/2022-07-12.png)

https://github.com/rsuu/rmg/blob/main/assets/2022-06-29.mp4

## Install

+ Dependency
  + Linux: x11 OR wayland
  + Windows: None
  + Mac: None(I DO NOT KNOW)
+ Optional Dependency
  + tar
  + zip
  + libheif

```bash
# github
Check here: https://github.com/rsuu/rmg/releases/

# cargo
cargo install rmg

# git
git clone --depth 1 https://github.com/rsuu/rmg
cd rmg
cargo build --release
```

## Usage

```bash
rmg ./tests/files/1.tar

# OR
rmg --size 600,600 ./tests/files/1.tar

# OR
rmg --size 600,600 --config ./tests/files/config.rs ./tests/files/1.tar
```

### KeyMap

|#|#|
|:-|:-|
k/Up | up
j/Down | down
h/Left | left
r/Right | right
q | quit

### Configuration

> config file: https://raw.githubusercontent.com/rsuu/rmg/main/tests/files/config.rs

WARN: You have to create the file by yourself.

+ configuration file path
  + Linux: `$HOME/.config/rmg/config.rs`
  + Windows: `C:\Users\Alice\AppData\<USER>\rmg\config.rs`
  + Mac: `$HOME/Library/Application Support/rmg/config.rs`

## Features

```bash
# Add support for heic
cargo run --release -F "de_heic"

# set [CpuExtensions](https://docs.rs/fast_image_resize/latest/fast_image_resize/index.html#resize-rgb8-image-u8x3-4928x3279--852x567)
cargo run --release -F "avx2"
    # for avx2
cargo run --release -F "sse4_1"
    # for sse4_1
```
