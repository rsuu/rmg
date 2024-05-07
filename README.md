# rmg

**Rust: Tiny And Fast Manga/Image Viewer**

## Demo

![](./assets/demo.jpg)

## Install

+ Dependency
  + Linux: X11/Wayland
  + Mac: -
  + Windows: -
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

rmg file.zip

# rmg file.gif

rmg --canvas-size 600x600 file.zip

rmg --canvas-size 600x600 --config ./assets/config.rs file.zip
```

## Keymap

|Key     | Action              |
|:-      |:-                   |
|k/Up    | move up             |
|j/Down  | move down           |
|h/Left  | move left           |
|r/Right | move right          |
|g       | toggle gesture mode |
|q       | quit                |

## Mouse Binding

|Key          | Action    |  Mode       |
|:-           |:-         |:-           |
| scroll up   | move up   | Vertical    |
| scroll down | move down | Vertical    |
| scroll up   | zoom out  | Single      |
| scroll down | zoom in   | Single      |

## Gesture

|Gesture| Action   |
|:-     |:-        |
|rect   |          |
|ring   |          |

## Config

> config: https://raw.githubusercontent.com/rsuu/rmg/main/assets/config.rs

NOTE: You must create the file first.

+ Linux: `$HOME/.config/rmg/config.rs`
+ Mac: `$HOME/Library/Application Support/rmg/config.rs`
+ Windows: `C:\Users\<USER>\AppData\<USER>\rmg\config.rs`

see more: https://docs.rs/dirs-next/

## Supported Formats

| Format    | Supported | Default | Dependency|
|:-         |:-         |:-       |:-         |
|.jpg       | +         | +       |           |
|.png       | +         | +       |           |
|.webp      | +         | +       |           |
|.heic/avif | +         |         | libheif   |
|.gif       | (dev)     | +       |           |
|.aseprite  | (dev)     |         |           |
|.svg       | (dev)     |         |           |

| Format   | Supported | Default | Dependency|
|:-        |:-         |:-       |:-         |
|directory | +         | +       |           |
|.tar      | +         |         |           |
|.zip/cbz  | +         | +       |           |
|.7z       |           |         |           |
|.rar      |           |         |           |
|.zst      |           |         |           |

## Features

```bash
# HEIF/HEIC
cargo run --release -F de_heic

# SVG and Aseprite
cargo run --release -F de_svg,de_aseprite

# see [CpuExtensions](https://docs.rs/fast_image_resize/latest/fast_image_resize/index.html#resize-rgb8-image-u8x3-4928x3279--852x567)
cargo run --release -F arch_avx2
    # AVX2
cargo run --release -F arch_sse4_1
    # SSE4_1

cargo run --release -F full
```
