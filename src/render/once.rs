use crate::{
    render::{
        keymap::{self, KeyMap, Map},
        scroll::Scroll,
        window::Canvas,
        {Data, Page},
    },
    sleep,
};

#[derive(Debug)]
pub struct Once {
    buffer: Vec<u32>,
    buffer_size: usize,
    page: Page,
    page_loading: Vec<u32>,

    y_step: usize,
    rng: usize,
    bit_len: usize,
}

impl Once {
    pub fn from_scroll(scroll: Scroll) -> Self {
        Self {
            buffer_size: scroll.buffer_size,
            y_step: scroll.y_step,
            page: scroll.page_list.list[0].clone(),
            page_loading: scroll.page_loading,
            rng: 0,
            bit_len: 0,
            buffer: vec![],
        }
    }

    pub fn start(&mut self, canvas: &mut Canvas, keymaps: &[KeyMap], data: &Data) {
        self.page.load_file(data).expect("ERROR: load_file()");
        self.bit_len = self.page.img.len();

        'l1: while canvas.window.is_open() {
            match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
                Map::Down => {
                    self.move_down();
                }

                Map::Up => {
                    self.move_up();
                }

                Map::Exit => {
                    println!("EXIT");

                    break 'l1;
                }

                _ => {}
            }

            self.buffer.clear();

            if self.page.flush(&mut self.buffer) {
                self.page.img.to_next_frame();
            }

            while self.buffer.len() < self.end() {
                self.buffer.extend_from_slice(&self.page_loading);
            }
            self.buffer.clear();
            //self.buffer.shrink_to(0);

            canvas.flush(&self.buffer[self.rng..self.end()]);

            sleep()
        }
    }

    /// move down
    fn move_down(&mut self) {
        if self.bit_len >= self.end() + self.y_step {
            self.rng += self.y_step;
        } else if self.bit_len >= self.buffer_size {
            self.rng = self.bit_len - self.buffer_size;
        }
    }

    /// move up
    fn move_up(&mut self) {
        if self.rng >= self.y_step {
            self.rng -= self.y_step;
        } else {
            self.rng = 0;
        }
    }

    fn end(&self) -> usize {
        self.rng + self.buffer_size
    }
}
