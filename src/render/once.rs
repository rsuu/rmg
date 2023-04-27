use crate::{keymap, sleep, Canvas, Config, Data, KeyMap, Map, Page, Scroll};

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

    pub fn start(&mut self, canvas: &mut Canvas, keymaps: &[KeyMap], data: &Data, config: &Config) {
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

                _ => {
                    self.mouse_input(canvas, config);
                }
            }

            self.buffer.clear();
            //self.buffer.shrink_to(0);

            if self.page.flush(&mut self.buffer) {
                self.page.img.to_next_frame();
            }

            while self.buffer.len() < self.end() {
                self.buffer.extend_from_slice(&self.page_loading);
            }

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

    #[inline(always)]
    fn mouse_input(&mut self, canvas: &mut Canvas, config: &Config) {
        // scroll
        if let Some((_, y)) = canvas.window.get_scroll_wheel() {
            //tracing::trace!("mouse_y == {}", y);

            match config.base.invert_mouse {
                true if y < 0.0 => self.move_up(),
                true if y > 0.0 => self.move_down(),

                false if y < 0.0 => self.move_down(),
                false if y > 0.0 => self.move_up(),

                _ => {}
            }
        }
    }
}
