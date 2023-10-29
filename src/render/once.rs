use crate::{keymap, sleep, Canvas, Config, Data, KeyMap, Map, Page, Scroll};

#[derive(Debug)]
pub struct Once {
    buffer: Vec<u32>,
    buffer_len: usize,
    page: Page,
    page_loading: Vec<u32>,

    y_step: usize,
    rng: usize,
    bit_len: usize,
}

impl Once {
    pub fn from_scroll(scroll: Scroll) -> Self {
        Self {
            buffer_len: scroll.buffer_len,
            y_step: scroll.y_step,
            page: scroll.page_list.list[0].clone(),
            page_loading: scroll.page_loading,
            rng: 0,
            bit_len: 0,
            buffer: vec![],
        }
    }

    pub fn start(
        &mut self,
        canvas: &mut Canvas,
        keymaps: &[KeyMap],
        data: &Data,
        config: &Config,
    ) -> anyhow::Result<()> {
        self.page.load_file(data)?;
        self.bit_len = self.page.len();

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

        Ok(())
    }

    /// move down
    fn move_down(&mut self) {
        if self.bit_len >= self.end() + self.y_step {
            self.rng += self.y_step;
        } else if self.bit_len >= self.buffer_len {
            self.rng = self.bit_len - self.buffer_len;
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
        self.rng + self.buffer_len
    }

    #[inline(always)]
    fn mouse_input(&mut self, canvas: &mut Canvas, config: &Config) {
        // scroll
        let Some((.., y)) = canvas.window.get_scroll_wheel() else {
            return;
        };

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
