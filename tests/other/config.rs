fn main() {}

fn base() -> Res {
    Res {
        size: (900, 900),
        font: None,
        rename_pad: 6,
        invert_mouse: false,
        filter: "Hamming",
        step: 6,
        limit: 4,
    }
}

fn keymap() -> Res {
    Res {
        up: 'k',
        down: 'j',
        left: 'h',
        right: 'l',
        exit: 'q',
    }
}

fn window() -> Res {
    Res {
        borderless: false,
        topmost: false,
        resize: false,
        none: true,
    }
}
