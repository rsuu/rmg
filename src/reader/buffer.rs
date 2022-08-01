use crate::{
    archive,
    color::{format::PixelFormat, rgb::TransRgb, rgba::TransRgba},
    img::{
        resize::{self, resize_bytes},
        size::Size,
    },
    reader::{keymap::Map, mini::Canvas2},
    utils::types::{ArchiveType, MyResult},
};
use log::debug;
use minifb::Key;
use std::{borrow::BorrowMut, ptr};

use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PageInfo {
    pub path: PathBuf,
    pub name: String,
    pub len: usize,
    pub pos: usize,
}

impl PageInfo {
    pub fn new(path: PathBuf, name: String, len: usize, pos: usize) -> Self {
        PageInfo {
            path,
            name,
            len,
            pos,
        }
    }
}

#[derive(Debug)]
pub struct Buffer {
    pub bytes: Vec<u32>,
    pub max_bytes: usize,
    pub start: usize,
    pub end: usize,

    pub page_list: Vec<PageInfo>,
    pub page_end: usize,

    pub archive_path: PathBuf,
    pub archive_type: ArchiveType,

    pub block: usize,
    pub step: usize,

    pub mode: Map,
    pub format: PixelFormat,

    pub window_size: Size<u32>,
    pub screen_size: Size<u32>,
    pub window_position: (i32, i32),

    pub view: (usize, usize),
}

impl Buffer {
    pub fn init(&mut self, canvas: &mut Canvas2) {
        if !self.page_list.is_empty() {
            self.next(self.view.1); // view: (0,0)
        } else {
            panic!()
        }
        'l1: while self.need_pad() && self.not_tail() {
            self.load_next();
        }
    }

    /// goto next page
    pub fn move_down(&mut self, canvas: &mut Canvas2) {
        self.try_free(canvas);
        self.inline_move_down(canvas);

        self.mode = Map::Down;

        log::info!("move_down()");
    }

    /// goto prev page
    pub fn move_up(&mut self, canvas: &mut Canvas2) {
        // BUG:

        self.try_free(canvas);
        self.inline_move_up(canvas);

        self.mode = Map::Up;

        log::info!("move_up()");
    }

    pub fn move_left(&mut self) {
        // NOTE: overflow
        if self.bytes.len() > self.end + 100 {
            self.start += 100;
            self.end += 100;

            println!("start: {}", self.start);
            println!("end: {}", self.end);
        } else {
        }

        self.mode = Map::Left;
    }

    pub fn move_right(&mut self) {
        // MAYBE BUG:
        if self.start >= 100 {
            self.start -= 100;
            self.end -= 100;
        } else if self.start >= 0 {
            let s = self.start;
            self.start -= s;
            self.end -= s;
        } else {
        }

        self.mode = Map::Right;
    }

    #[inline]
    pub fn inline_move_down(&mut self, canvas: &mut Canvas2) {
        if self.bytes.len() > self.end + self.block {
            self.start += self.block;
            self.end += self.block;
        } else if self.bytes.len() >= self.end {
            self.start += self.bytes.len() - self.end;
            self.end += self.bytes.len() - self.end;
        } else {
        }
    }

    #[inline]
    pub fn inline_move_up(&mut self, canvas: &mut Canvas2) {
        if self.start >= self.block {
            self.start -= self.block;
            self.end -= self.block;
        } else if self.start >= 0 {
            if self.not_head() {
                self.load_prev();

                if self.start >= self.block && self.start >= self.max_bytes {
                    self.start -= self.block;
                    self.end -= self.block;
                } else {
                    let s = self.start;
                    self.start -= s;
                    self.end -= s;
                }
            } else {
                let s = self.start;
                self.start -= s;
                self.end -= s;
            }
        } else {
        }
    }

    pub fn load_prev(&mut self) {
        self.goto_prev();
        self.prev(self.view.0);
    }

    pub fn load_next(&mut self) {
        self.goto_next();
        self.next(self.view.1);
    }

    pub fn prev(&mut self, pos: usize) {
        let mut bytes = Vec::new();
        let mut buffer = Vec::new();

        self.load_img(&mut bytes, pos);

        for f in (0..bytes.len()).step_by(3) {
            buffer.push(TransRgb::rgb_to_u32(&bytes[f..f + 3].try_into().unwrap()));
        }

        push_front(&mut self.bytes, buffer.as_slice());

        self.start += buffer.len();
        self.end += buffer.len();
    }

    pub fn next(&mut self, pos: usize) {
        let mut bytes = Vec::new();
        let mut buffer = Vec::new();

        self.load_img(&mut bytes, pos);

        // for f in buffer.as_slice().chunks(3) {}
        for f in (0..bytes.len()).step_by(3) {
            buffer.push(TransRgb::rgb_to_u32(&bytes[f..f + 3].try_into().unwrap()));
        }

        self.bytes.extend_from_slice(buffer.as_slice());
        self.page_list[pos].len = buffer.len();
    }

    #[inline]
    pub fn try_free(&mut self, canvas: &mut Canvas2) {
        match self.mode {
            Map::Down => {
                let cut_len = self.page_list[self.view.0].len;

                if (self.at_tail() || self.need_pad()) && self.not_tail() {
                    self.load_next();

                    log::info!("next");
                } else if self.bytes.len() > self.max_bytes + cut_len && self.start >= cut_len {
                    self.view.0 += 1;
                    free_head(&mut self.bytes, cut_len);

                    self.start -= cut_len;
                    self.end -= cut_len;
                } else {
                }
            }
            Map::Up => {
                let cut_len = self.page_list[self.view.1].len;

                if (self.at_head() || self.need_pad()) && self.not_head() {
                    self.load_prev();

                    log::info!("prev: {}", self.bytes.len() % 2);
                } else if self.bytes.len() > self.max_bytes * 2 + cut_len
                    && self.view.1 - 1 >= 0
                    && self.bytes.len() > self.end + cut_len
                {
                    self.view.1 -= 1;
                    free_tail(&mut self.bytes, cut_len);

                    log::debug!("le:: bytes.len: {:?}", cut_len);
                } else {
                }
            }
            _ => {}
        }
    }

    pub fn load_img(&self, buffer: &mut Vec<u8>, pos: usize) {
        let bytes = match self.archive_type {
            Tar => archive::tar::load_file(
                self.archive_path.as_path(),
                self.page_list[pos].path.as_path(),
            )
            .unwrap(),

            Zip => {
                todo!()
            }

            Zstd => {
                todo!()
            }
        };

        resize_bytes(bytes.as_slice(), buffer, self.screen_size, self.window_size);
    }

    #[inline(always)]
    pub fn flush(&self, canvas: &mut Canvas2) {
        canvas.flush(&self.bytes[self.start..self.end]);
    }

    pub fn at_head(&self) -> bool {
        self.start == 0
    }

    pub fn at_tail(&self) -> bool {
        self.end == self.bytes.len()
    }

    pub fn not_head(&self) -> bool {
        self.view.0 >= 1
    }

    pub fn not_tail(&self) -> bool {
        self.view.1 + 1 < self.page_end
    }

    pub fn need_pad(&self) -> bool {
        self.bytes.len() < self.max_bytes * 2
    }

    pub fn goto_prev(&mut self) {
        self.view.0 -= 1;
    }

    pub fn goto_next(&mut self) {
        self.view.1 += 1;
    }
}

#[inline]
pub fn push_front<T>(vec: &mut Vec<T>, slice: &[T])
where
    T: Copy,
{
    unsafe {
        let len = vec.len();
        let amt = slice.len();

        vec.reserve(amt);

        ptr::copy(vec.as_ptr(), vec.as_mut_ptr().offset((amt) as isize), len);
        ptr::copy(slice.as_ptr(), vec.as_mut_ptr(), amt);

        vec.set_len(len + amt);
    }
}

pub fn free_head<T>(buffer: &mut Vec<T>, range: usize)
where
    T: Sized + Clone,
{
    buffer.drain(..range);
}

pub fn free_tail<T>(buffer: &mut Vec<T>, range: usize)
where
    T: Sized,
{
    buffer.truncate(buffer.len() - range);
}

mod test {
    use super::*;

    #[test]
    fn _push_front() {
        let mut a = vec![4, 5, 6];
        push_front(&mut a, [1, 2, 3].as_slice());

        assert_eq!(a.as_slice(), [1, 2, 3, 4, 5, 6].as_slice());
    }
}
