// TODO:
//   CI
//   main()

use console_error_panic_hook;
use minifb::{Key, Window, WindowOptions};
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

const WIDTH: usize = 300;
const HEIGHT: usize = 200;

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut window = Window::new("Bouncy Box demo", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut buf = {
        #[cfg(feature = "web")]
        {
            // ABGR
            vec![0xFF42F5AD; WIDTH * HEIGHT]
        }

        #[cfg(not(feature = "web"))]
        {
            // ARGB
            vec![0xFFADF542; WIDTH * HEIGHT]
        }
    };

    window.update_with_buffer(&buf, WIDTH, HEIGHT).unwrap();

    // A reference counted pointer to the closure that will update and render the game
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *(g.borrow_mut()) = Some(Closure::wrap(Box::new(move || {
        if window.is_key_down(Key::J) {
            for y in (0..HEIGHT / 2) {
                for x in (0..WIDTH / 2) {
                    buf[WIDTH * y + x] = 0xFFaabbcc;
                }
            }
        } else if window.is_key_down(Key::K) {
            for y in (0..HEIGHT / 2) {
                for x in (0..WIDTH / 2) {
                    buf[WIDTH * y + x] = 0xFF112233;
                }
            }
        } else {
            buf = vec![0xFF42F5AD; WIDTH * HEIGHT];
        }

        window.update_with_buffer(&buf, WIDTH, HEIGHT).unwrap();
        // schedule this closure for running again at next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() + 'static>));

    // start the animation loop
    request_animation_frame(g.borrow().as_ref().unwrap());
}

// TODO: http GET /config.rs
pub fn try_get_config() {}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
