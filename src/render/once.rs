use crate::{
    render::{
        keymap::{self, KeyMap, Map},
        scroll::Scroll,
        {Data, Page},
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
                self.page.img.to_next_frame();
            } else {
                panic!("");
            }

            while buffer.len() < rng + self.buffer_max {
                buffer.extend_from_slice(&self.page_loading);
            }

            buffer.truncate(rng + self.buffer_max);
            canvas.flush(&buffer[rng..rng + self.buffer_max]);

            sleep_ms(6);
        }
    }
}
