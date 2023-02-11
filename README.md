# rmg

> Rust: Tiny And Fast Manga/Image Viewer

## Demo

![](./assets/demo.jpg)

./assets/demo.mp4

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
rmg tests/bit/png

rmg file.tar

rmg --size 600x600 file.tar

rmg --size 100x600 --config ./tests/other/config.rs file.tar

rmg file.gif
```

### KeyMap

|Key| |
|:-|:-|
k/Up | move up
j/Down | move down
h/Left | move left
r/Right | move right
q | quit

### Configuration

> config file: https://raw.githubusercontent.com/rsuu/rmg/main/tests/other/config.rs

`WARN:` You have to create the file by yourself.

+ configuration file path
  + Linux: `$HOME/.config/rmg/config.rs`
  + Mac: `$HOME/Library/Application Support/rmg/config.rs`
  + Windows: `C:\Users\<USER>\AppData\<USER>\rmg\config.rs`

## Supported formats

| Format | Supported | Default |Dependency | Mode
|:-|:-|:-|:-|:-|
.jpg |✅ | ✅||Scroll/Once
.png|✅| ✅||Scroll/Once
.heic / .avif|🔬|❌|libheif|Scroll/Once
.gif|🔬|✅||Once
.aseprite|🔬|❌||Once
.svg|🔬|❌||Scroll/Once

---
| Format | Supported | Default |Dependency
|:-|:-|:-|:-|
directory |✅ | ✅|
.tar |✅ | ✅| tar
.zip / .cbz |✅ | ✅| zip

## Features

```bash
# Add support for heic
cargo run --release -F "de_heic"

# for svg AND aseprite
cargo run --release -F "de_svg" -F "de_aseprite"

# [CpuExtensions](https://docs.rs/fast_image_resize/latest/fast_image_resize/index.html#resize-rgb8-image-u8x3-4928x3279--852x567)
cargo run --release -F "avx2"
    # for avx2
cargo run --release -F "sse4_1"
    # for sse4_1
```

## TIPS

+ > floating window in wayland
  + `~/.config/sway/config`
  + for_window [title="rmg"]  floating enable

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
