use crate::{
    archive::ArchiveType,
    reader::{
        scroll::Scroll,
        view::{Buffer, Check, Data, Page, PageList},
    },
    utils::traits::AutoLog,
    FPS,
};
use log::debug;
use std::{mem::swap, path::PathBuf};

use super::{keymap, keymap::Map, window::Canvas};
use crate::reader::keymap::KeyMap;

#[derive(Debug)]
pub struct Turn {
    pub buffer: Buffer,
    pub buffer_max: usize,

    pub page_list: PageList,

    pub cur: usize, //
    pub map: Map,

    pub rng: usize,
    pub y_step: usize,

    pub is_double_page: bool,
    pub is_manga: bool,

    pub page_max: usize,
}

impl Turn {
    pub fn from_scroll(scroll: Scroll) -> Self {
        Self {
            buffer: Buffer::new(),
            buffer_max: scroll.buffer_max,
            page_list: scroll.page_list,
            cur: 1,
            map: Map::Right,
            rng: 0,
            y_step: scroll.y_step, // drop 1/step part of image once
            is_double_page: false,
            is_manga: false,
            page_max: 3,
        }
    }

    pub fn start(&mut self, canvas: &mut Canvas, keymaps: &[KeyMap]) {
        let mut time_start = std::time::Instant::now();
        let mut sleep = FPS;

        'l1: while canvas.window.is_open() {
            match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
                Map::Down => self.move_down(),

                Map::Up => self.move_up(),

                Map::Left => self.goto_prev_page(),

                Map::Right => self.goto_next_page(),

                Map::Exit => break 'l1,

                _ => {}
            }
        }

        self.flush(canvas);
        self.to_next_frame();

        let now = std::time::Instant::now();
        let count = (now - time_start).as_millis() as u64;

        time_start = now;
        sleep = FPS.checked_sub(count / 6).unwrap_or(10);

        std::thread::sleep(std::time::Duration::from_millis(sleep));
    }

    pub fn flush(&mut self, canvas: &mut Canvas) {
        self.buffer.flush(self.page_list.get(self.cur).data());

        if self.cur_len() > self.buffer_max {
            self.buffer
                .data
                .extend_from_slice(&vec![0; self.cur_len() - self.buffer_max]);
        } else {
        };

        canvas.flush(&self.buffer.data[self.rng..self.rng + self.buffer_max]);
    }

    pub fn sort_page(&mut self) {
        if self.is_manga {
            let count = self.page_list.len() / 2;
            let (mut l, mut r) = (0, 1);

            for _ in 0..count {
                self.page_list.swap(l, r);

                l += 2;
                r += 2;
            }
        } else {
        }
    }

    pub fn not_tail(&self) -> bool {
        self.page_list.get(self.cur + 1).check != Check::Tail
    }

    pub fn not_head(&self) -> bool {
        self.page_list.get(self.cur - 1).check != Check::Head
    }

    pub fn get_cur_mut(&mut self) -> &mut Page {
        self.page_list.get_mut(self.cur)
    }

    pub fn get_cur(&self) -> &Page {
        self.page_list.get(self.cur)
    }

    pub fn cur_len(&self) -> usize {
        self.page_list.get(self.cur).len()
    }

    pub fn goto_next_page(&mut self) {
        if self.not_tail() {
            //self.try_free_page_prev( self.cur - 1 );
            //self.try_load_page_next( self.cur + 1 );

            self.rng = 0;
            self.cur += 1;
        }
    }

    pub fn goto_prev_page(&mut self) {
        if self.not_head() {
            self.rng = 0;
            self.cur -= 1;
        }
    }

    pub fn try_load_page_next(&mut self) {
        for f in 0..self.page_max {}
    }

    pub fn try_load_page_prev(&mut self) {
        for f in 0..self.page_max {}
    }

    pub fn free_page(&mut self) {}

    /// move up
    pub fn move_up(&mut self) {
        "MOVE UP"._info();

        self.map = Map::Up;

        if self.rng >= self.y_step {
            self.rng -= self.y_step;
        } else {
            // if (self.rng >= 0)
            self.rng = 0;
        };
    }

    /// move down
    pub fn move_down(&mut self) {
        "MOVE DOWN"._info();
        debug!("{}    --    {}", self.rng, self.buffer.len());

        self.map = Map::Down;

        // scrolling
        if self.rng + self.y_step <= self.buffer.len() - self.buffer_max {
            self.rng += self.y_step;
        } else {
            // if (self.rng <= self.buffer_len - self.buffer_max)
            self.rng = self.buffer.len() - self.buffer_max;
        };
    }

    pub fn to_next_frame(&mut self) {
        self.get_cur_mut().to_next_frame()
    }
}
