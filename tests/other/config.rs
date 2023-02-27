fn main() {
    Base {
        size: (900, 900),
        rename_pad: 6,
        invert_mouse: false,
        filter: "Hamming",
        step: 6,
        limit: 5,
        font: None,
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
