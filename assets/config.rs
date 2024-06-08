fn main() {}

fn app() -> Any {
    Any { target: "./" }
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
        layout: Layout::Vertical {
            align: Align::Center,
        },

        cache_limit: 2,
        pre_load_nums: 4,

        /// RGBA
        bg: 0x00_00_00_00,
    }
}

fn page() -> Any {
    Any {
        size: Size {
            width: 800,
            height: 600,
        },
        img_resize_algo: WrapResizeAlg::Lanczos3,
        anim_resize_algo: WrapResizeAlg::Nearest,
    }
}

fn misc() -> Any {
    Any {
        padding_filename: 10,
    }
}

fn once() -> Any {
    Any {
        record_gesture_name: None,
    }
}

fn gestures() -> Any {
    Any {
        data_path: "./assets/gestures.zip",
        min_score: 0.9,
    }
}

fn layout_double() -> Any {
    Any {
        /// right to left
        reading_dire: Direction::Rtl,
    }
}

fn on_scroll() -> Any {
    Any {
        step_x: 100.0,
        step_y: 100.0,
    }
}
