# rmg

Mnaga Reader

## Install

+ Dependency
  + Linux: x11 OR wayland
+ Optional Dependency
  + tar
  + zip

```bash
cargo install rmg

#OR

git clone --depth 1 https://github.com/rsuu/rmg
cd rmg
cargo build --release
```


## Usage

```bash
cargo run -- --config ./tests/files/config.rs --size 600,600 ./tests/files/img.zip

# OR

rmg --config ./tests/files/config.rs --size 600,600 ./tests/files/img.zip

# OR

rmg --config ./tests/files/config.rs ./tests/files/img.zip


```

### KeyMap

|#|#|
|:-|:-|
j | down
k | up
r | ? reset
f | ? fullscreen
q | exit

## Demo

![](./assets/2022-07-12.png)
https://github.com/rsuu/rmg/blob/main/assets/2022-06-29.mp4
