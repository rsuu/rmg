fn main() {
    Base {
        size: (900, 900),
        font: "./tests/files/test.ttf",
        format: "rgb8",
        rename: true,
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
