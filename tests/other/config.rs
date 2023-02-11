fn main() {
    Base {
        size: (900, 900),
        font: None,
        rename_pad: 6,
        invert_mouse: false,
        filter: "Hamming",
        step: 5,
        limit: 10,
    };

    Keymap {
        up: 'k',
        down: 'j',
        left: 'h',
        right: 'l',
        exit: 'q',
    };

    Window {
        borderless: false,
        topmost: false,
        resize: false,
        none: true,
    }
}
