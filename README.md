# rmg

Mnaga Reader

## Install

+ Dependency
  + Linux: x11 OR wayland
  + Windows: None
  + Mac: None(I DO NOT KNOW)
+ Optional Dependency
  + tar
  + zip

```bash
cargo install rmg

# OR

git clone --depth 1 https://github.com/rsuu/rmg
cd rmg
cargo build --release

# OR
Downlaod here: https://github.com/rsuu/rmg/releases/
```


## Usage

```bash
cargo run -- --config ./tests/files/config.rs --size 600,600 ./tests/files/1.tar

# OR

rmg --config ./tests/files/config.rs --size 600,600 ./tests/files/1.tar

# OR

rmg --config ./tests/files/config.rs ./tests/files/1.tar

# OR

rmg ./tests/files/1.tar


```

### KeyMap

|#|#|
|:-|:-|
j | down
k | up
r | ? reset
f | ? fullscreen
q | exit

### Configuration

> default configuration

```text
fn main() {
    Base {
        size: (900, 900),
        font: None,
        rename_pad: 6,
    };

    Keymap {
        up: 'k',
        down: 'j',
        left: 'h',
        right: 'l',
        exit: 'q',
    };
}
```

+ configuration file path
  + Linux: `$HOME/.config/rmg/config.rs`
  + Windows: `C:\Users\Alice\AppData\<USER>\rmg\config.rs`
  + Mac: `$HOME/Library/Application Support/rmg/config.rs`

note: You should create the file by yourself.

## Demo

![](./assets/2022-07-12.png)

https://github.com/rsuu/rmg/blob/main/assets/2022-06-29.mp4
