fn main() {
    Base {
        size: (900, 900),    // width AND height
        font: None,          //
        rename_pad: 0,       //
        invert_mouse: false, //
        filter: "Hamming",   // ["Box", "Hamming", ]
        step: 10,            //
    };

    Keymap {
        up: 'k',
        down: 'j',
        left: 'h',
        right: 'l',
        exit: 'q',
    };
}
