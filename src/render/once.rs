use crate::{
    render::{
        keymap::{self, KeyMap, Map},
        scroll::Scroll,
        utils::{Data, Page},
        window::Canvas,
    },
    FPS,
};
use std::thread::sleep_ms;

#[derive(Debug)]
pub struct Once {
    buffer_max: usize,
    y_step: usize,
    page: Page,
    page_loading: Vec<u32>,
}

impl Once {
    pub fn from_scroll(scroll: Scroll) -> Self {
        Self {
            buffer_max: scroll.buffer_max,
            y_step: scroll.y_step,
            page: scroll.page_list.list[0].clone(),
            page_loading: scroll.page_loading,
        }
    }

    pub fn start(&mut self, canvas: &mut Canvas, keymaps: &[KeyMap], data: &Data) {
        let mut time_start = std::time::Instant::now();
        let mut ms = FPS;

        self.page.load_file(data).expect("ERROR: load_file()");

        let mut buffer: Vec<u32> = vec![];
        let mut rng = 0;

        'l1: while canvas.window.is_open() {
            match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
                Map::Down => {
                    // scrolling
                    if rng + self.y_step <= buffer.len() - self.buffer_max {
                        rng += self.y_step;
                    } else {
                        rng = buffer.len() - self.buffer_max;
                    };
                }

                Map::Up => {
                    if rng >= self.y_step {
                        rng -= self.y_step;
                    } else {
                        // if (rng >= 0)
                        rng -= rng;
                    };
                }

                Map::Exit => {
                    println!("EXIT");

                    break 'l1;
                }

                _ => {}
            }

            buffer.clear();
            if self.page.flush(&mut buffer) {
                self.page.to_next_frame();
            } else {
                panic!("");
            }

            while buffer.len() < rng + self.buffer_max {
                buffer.extend_from_slice(&self.page_loading);
            }

            canvas.flush(&buffer[rng..rng + self.buffer_max]);

            let now = std::time::Instant::now();
            let count = (now - time_start).as_millis() as u32;
            time_start = now;
            ms = FPS.checked_sub(count / 6).unwrap_or(10);

            sleep_ms(ms);
        }
    }
}
