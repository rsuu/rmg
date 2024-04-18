fn main() {}

fn app() -> Any {
    Any { path: "./" }
}

fn window() -> Any {
    Any {
        borderless: false,
        fullscreen: false,
        auto_resize: false,
        invert_mouse: false,
    }
}

fn canvas() -> Any {
    Any {
        size: Size {
            width: 800,
            height: 600,
        },

        layout: Layout::Vertical,

        step_x: 100.0,
        step_y: 100.0,

        cache_limit: 2,
        mode: Mode::Manga,

        // RGBA
        bg: 0x00_00_00_00,
    }
}

fn page() -> Any {
    Any {
        image_resize_algo: WrapResizeAlg::Lanczos3,
        anime_resize_algo: WrapResizeAlg::Nearest,
    }
}

fn misc() -> Any {
    Any {
        padding_filename: 10,
    }
}
