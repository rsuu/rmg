use minifb::{Key, Scale, ScaleMode, Window};

pub struct Canvas2 {
    pub window: Window,
    pub size: (usize, usize),
}

impl Canvas2 {
    pub fn new(width: usize, height: usize) -> Self {
        let windowoptions = minifb::WindowOptions {
            borderless: false,
            transparency: false,
            title: true,
            resize: true,
            topmost: true,
            none: false,
            scale_mode: ScaleMode::Center,
            scale: Scale::X1,
        };

        Self {
            window: Window::new("rmg", width, height, windowoptions).unwrap(),
            size: (width, height),
        }
    }

    #[inline(always)]
    pub fn flush(&mut self, data: &[u32]) {
        self.window
            .update_with_buffer(data, self.size.0, self.size.1)
            .unwrap();
    }
}

pub fn test_input() {
    let mut window = Window::new(
        "Noise Test - Press ESC to exit",
        400,
        400,
        minifb::WindowOptions {
            borderless: true,
            transparency: true,
            title: false,
            resize: true,
            topmost: false,
            none: false,
            scale_mode: ScaleMode::Center,
            scale: Scale::X1,
        },
    )
    .expect("Unable to open Window");

    window.set_position(0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        println!("ddd");
        window
            .update_with_buffer(&[0; 400 * 400], 400, 400)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
