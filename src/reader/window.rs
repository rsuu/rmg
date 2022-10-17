use minifb::{Scale, ScaleMode, Window};

pub struct Canvas {
    pub window: Window,
    pub size: (usize, usize),
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let windowoptions = minifb::WindowOptions {
            borderless: false,
            transparency: false,
            title: true,
            resize: false,
            topmost: false, // https://github.com/tauri-apps/tauri/issues/3117#issuecomment-1027910946
            // After a bit of research, a lot of resources seems to indicate that
            // Wayland doesn't have an api for setting alwayOnTop so we don't have
            // much choice but to wait for Wayland to add an api for it.
            none: true,
            scale_mode: ScaleMode::Center,
            scale: Scale::X1,
        };

        let mut window = Window::new("rmg", width, height, windowoptions).unwrap();

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // Limit to max ~60 fps update rate

        //window.set_position(720, 0);

        Self {
            window,
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

//mod test {
//
//    #[test]
//    pub fn test_input() {
//        let mut window = Window::new(
//            "Noise Test - Press ESC to exit",
//            400,
//            400,
//            minifb::WindowOptions {
//                borderless: true,
//                transparency: true,
//                title: false,
//                resize: true,
//                topmost: false,
//                none: false,
//                scale_mode: ScaleMode::Center,
//                scale: Scale::X1,
//            },
//        )
//        .expect("Unable to open Window");
//
//        window.set_position(0, 0);
//
//        //while window.is_open() {
//        //window
//        //   .update_with_buffer(&[0; 400 * 400], 400, 400)
//        //  .unwrap();
//
//        //    std::thread::sleep(std::time::Duration::from_millis(100));
//        //}
//    }
//}
