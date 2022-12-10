fn main() {
    Base {
        size: (900, 900),    // width AND height
        font: None,          //
        rename_pad: 6,       //
        invert_mouse: false, //
        filter: "Lanczos3",  // [Box, Hamming, Lanczos3, CatmullRom, Mitchell]
        step: 4,             //
    };

    Keymap {
        up: 'k',
        down: 'j',
        left: 'h',
        right: 'l',
        exit: 'q',
    };
}
