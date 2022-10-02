fn main() {
    Base {
        size: (900, 900),
        font: None,
        rename_pad: 6,

        image_filter_type: "box",
        auto_resize: true,
        auto_topmost: false, // NOTE: only works on windows
    };

    Keymap {
        up: 'k',
        down: 'j',
        left: 'h',
        right: 'l',
        exit: 'q',
    };
}
