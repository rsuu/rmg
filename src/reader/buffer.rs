use crate::{
    archive,
    color::rgba::TransRgba,
    img::{resize, size::Size},
    reader::{keymap::Map, window::Canvas},
    utils::types::ArchiveType,
};

use fir;
use log;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use tokio;

static LOAD_MAX: usize = 2;

// use for async
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    Nothing,

    NextReady,
    NextDone,

    PrevReady,
    PrevDone,
}

/// Base Info
#[derive(Debug, Clone)]
pub struct PageInfo {
    pub name: String, // filename
    pub len: usize,   // width * heigth
    pub pos: usize,   // index of image in the archive file
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub bytes: Vec<u32>,
    pub max_bytes: usize,

    pub head: usize, // default: 0
    pub tail: usize, // default: 0

    pub start: usize,
    pub end: usize,

    pub page_list: Vec<PageInfo>,
    pub page_end: usize,

    pub archive_path: PathBuf,
    pub archive_type: ArchiveType,

    pub x_step: usize, // move_down AND move_up
    pub y_step: usize, // move_left AND move_right

    pub mode: Map, // [UP, DOWN, QUIT, ...]

    pub window_size: Size<u32>,
    pub screen_size: Size<u32>,
    pub window_position: (i32, i32),

    pub filter: fir::FilterType,
}

impl PageInfo {
    pub fn new(name: String, pos: usize) -> Self {
        PageInfo { name, len: 0, pos }
    }
}

impl Buffer {
    /// init
    pub fn init(&mut self) {
        // only works when page count >= 1
        if self.page_end >= 1 {
            self.load_next();

            'l1: while self.bytes.len() <= self.max_bytes * 2 && self.tail + 1 < self.page_end {
                self.tail += 1;
                self.load_next();
            }
        } else {
            panic!()
        }
    }

    #[inline(always)]
    pub fn flush(&self, canvas: &mut Canvas) {
        canvas.flush(&self.bytes[self.start..self.end]);

        // log::debug!("self.bytes.len() == {}", self.bytes.len());
    }

    pub fn load_next(&mut self) {
        log::info!("load next");

        let file_pos = self.page_list[self.tail].pos;

        let mut img_buf = Vec::new();
        get_buffer(
            &mut img_buf,
            self.archive_type,
            self.archive_path.as_path(),
            file_pos,
            self.screen_size,
            self.window_size,
            self.filter,
        );

        self.bytes.extend_from_slice(img_buf.as_slice());
        self.page_list[self.tail].len = img_buf.len();
    }

    /// goto next page
    // HACK: async version
    pub fn move_down(
        &mut self,
        color_buffer_arc: &Arc<RwLock<Vec<u32>>>,
        state_arc: &Arc<RwLock<State>>,
    ) {
        log::debug!(
            "
state: {:?}
start: {}
end: {}
len: {}
head: {}
tail: {}
",
            *state_arc.read().unwrap(),
            self.start,
            self.end,
            self.bytes.len(),
            self.head,
            self.tail,
        );

        let cut_len = self.page_list[self.head].len;

        // try to load next page
        if self.end >= self.bytes.len() / 8
            && self.tail + 1 < self.page_end
            && *state_arc.read().unwrap() == State::Nothing
        {
            self.tail += 1;

            let color_buf = color_buffer_arc.clone();
            let state = state_arc.clone();
            *state.write().unwrap() = State::NextReady;

            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let page_pos = self.page_list[self.tail].pos;
            let screen_size = self.screen_size;
            let window_size = self.window_size;
            let filter = self.filter;

            let join = tokio::spawn(async move {
                let mut img_selffer = Vec::new();

                get_buffer(
                    &mut img_selffer,
                    archive_type,
                    archive_path.as_path(),
                    page_pos,
                    screen_size,
                    window_size,
                    filter,
                );

                color_buf
                    .write()
                    .unwrap()
                    .extend_from_slice(img_selffer.as_slice());

                *state.write().unwrap() = State::NextDone;

                log::debug!("DONE");
            });

            drop(join);
        }

        // load next
        if *state_arc.read().unwrap() == State::NextDone {
            self.page_list[self.tail].len = color_buffer_arc.read().unwrap().len();
            self.bytes
                .extend_from_slice(color_buffer_arc.read().unwrap().as_slice());

            color_buffer_arc.write().unwrap().clear();

            log::debug!("state == {:?}", state_arc.read().unwrap());
            log::debug!("load next");

            *state_arc.write().unwrap() = State::Nothing;
        } else {
        }

        // try to free up the memory
        if *state_arc.write().unwrap() == State::Nothing
            && self.start > cut_len
            && self.bytes.len() >= self.max_bytes * 8 + cut_len
            && self.head + 1 < self.tail
        {
            self.head += 1;
            free_head(&mut self.bytes, cut_len);

            self.start -= cut_len;
            self.end -= cut_len;
        }

        // scrolling viewer
        if self.bytes.len() >= self.end + self.y_step {
            self.start += self.y_step;
            self.end += self.y_step;
        } else if self.bytes.len() >= self.end {
            self.start += self.bytes.len() - self.end;
            self.end += self.bytes.len() - self.end;
        } else {
        }

        // change the state
        self.mode = Map::Down;

        log::info!("move_down()");
    }

    /// goto prev page
    pub fn move_up(
        &mut self,
        color_buffer_arc: &Arc<RwLock<Vec<u32>>>,
        state_arc: &Arc<RwLock<State>>,
    ) {
        log::debug!(
            "
state: {:?}
start: {}
end: {}
len: {}
head: {}
tail: {}
",
            *state_arc.read().unwrap(),
            self.start,
            self.end,
            self.bytes.len(),
            self.head,
            self.tail,
        );

        let cut_len = self.page_list[self.tail].len;

        // try to load prev page
        if self.head >= 1
            && (self.start <= self.max_bytes * 2)
            && (*state_arc.read().unwrap() == State::Nothing
                || *state_arc.read().unwrap() == State::NextDone)
        {
            self.head -= 1;

            let color_buf = color_buffer_arc.clone();
            let state = state_arc.clone();
            *state.write().unwrap() = State::PrevReady;

            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let page_pos = self.page_list[self.head].pos;
            let screen_size = self.screen_size;
            let window_size = self.window_size;
            let filter = self.filter;

            let join = tokio::spawn(async move {
                let mut img_selffer = Vec::new();

                get_buffer(
                    &mut img_selffer,
                    archive_type,
                    archive_path.as_path(),
                    page_pos,
                    screen_size,
                    window_size,
                    filter,
                );

                color_buf
                    .write()
                    .unwrap()
                    .extend_from_slice(img_selffer.as_slice());

                *state.write().unwrap() = State::PrevDone;

                log::debug!("DONE");
            });

            drop(join);
        } else {
        }

        // load prev
        if *state_arc.read().unwrap() == State::PrevDone {
            push_front(&mut self.bytes, color_buffer_arc.read().unwrap().as_slice());

            let len = color_buffer_arc.read().unwrap().len();

            self.start += len;
            self.end += len;

            color_buffer_arc.write().unwrap().clear();
            *state_arc.write().unwrap() = State::Nothing;

            log::debug!("state == {:?}", state_arc.read().unwrap());
            log::debug!("load prev");
        } else {
        }

        // HACK: try to free up the memory
        if *state_arc.write().unwrap() == State::Nothing
            && self.bytes.len() >= self.end + cut_len
            && self.bytes.len() >= self.max_bytes * 8 + cut_len
            && self.tail > self.head + 1
        {
            self.tail -= 1;
            free_tail(&mut self.bytes, cut_len);

            log::debug!("move_up: free()");
        }

        if self.start >= self.y_step {
            self.start -= self.y_step;
            self.end -= self.y_step;
        } else if self.start >= 0 {
            let s = self.start;
            self.start -= s;
            self.end -= s;
        } else {
        }

        self.mode = Map::Up;

        log::info!("move_up()");
    }

    pub fn move_left(&mut self) {
        // HACK: overflow
        // ??? How it works
        if self.bytes.len() > self.end + self.x_step {
            self.start += self.x_step;
            self.end += self.x_step;

            log::debug!("start: {}", self.start);
            log::debug!("end: {}", self.end);
        } else {
        }

        self.mode = Map::Left;
    }

    ///
    pub fn move_right(&mut self) {
        // HACK: overflow
        if self.start >= self.x_step {
            self.start -= self.x_step;
            self.end -= self.x_step;
        } else {
        }

        self.mode = Map::Right;
    }

    ///
    pub fn need_pad_tail(&self) -> bool {
        self.bytes.len() <= self.max_bytes * LOAD_MAX
    }

    ///
    pub fn need_pad_head(&self) -> bool {
        self.bytes.len() <= self.max_bytes * LOAD_MAX
    }

    ///
    pub fn at_block_head(&self) -> bool {
        self.start == 0
    }

    ///
    pub fn at_block_tail(&self) -> bool {
        self.end == self.bytes.len()
    }

    ///
    pub fn not_page_tail(&self) -> bool {
        self.tail < self.page_end
    }

    ///
    pub fn not_next_page_tail(&self) -> bool {
        self.tail + 1 < self.page_end
    }

    ///
    pub fn to_prev_page(&mut self) {
        self.head -= 1;
    }
}

#[inline]
pub fn push_front<T>(vec: &mut Vec<T>, slice: &[T])
where
    T: ?Copy + Clone + Sized,
{
    let amt = slice.len(); // [1, 2, 3]
    let len = vec.len(); // [4, 5, 6]

    vec.reserve(amt);

    unsafe {
        std::ptr::copy(vec.as_ptr(), vec.as_mut_ptr().offset((amt) as isize), len);
        std::ptr::copy(slice.as_ptr(), vec.as_mut_ptr(), amt);

        vec.set_len(len + amt);
    }
}

#[inline]
pub fn free_head<T>(buffer: &mut Vec<T>, range: usize)
where
    T: Sized + Clone,
{
    buffer.drain(..range);
}

#[inline]
pub fn free_tail<T>(buffer: &mut Vec<T>, range: usize)
where
    T: Sized,
{
    buffer.truncate(buffer.len() - range);
}

///
pub fn get_buffer(
    buffer: &mut Vec<u32>,
    archive_type: ArchiveType,
    archive_path: &Path,
    page_pos: usize,
    screen_size: Size<u32>,
    window_size: Size<u32>,
    filter: fir::FilterType,
) {
    let mut img = Vec::new();

    resize_img(
        &mut img,
        archive_type,
        archive_path,
        page_pos,
        screen_size,
        window_size,
        &filter,
    );

    for f in (0..img.len()).step_by(4) {
        buffer.push(TransRgba::argb_to_u32(&img[f..f + 4].try_into().unwrap()));
    }
}

pub fn resize_img(
    buffer: &mut Vec<u8>,
    archive_type: ArchiveType,
    archive_path: &Path,
    page_pos: usize,
    screen_size: Size<u32>,
    window_size: Size<u32>,
    filter: &fir::FilterType,
) {
    log::debug!("archive_type == {:?}", archive_type);

    let bytes = match archive_type {
        ArchiveType::Tar => {
            log::debug!("ex_tar()");

            archive::tar::load_file(archive_path, page_pos).unwrap()
        }

        ArchiveType::Zip => {
            log::debug!("ex_zip()");

            archive::zip::load_file(archive_path, page_pos).unwrap()
        }

        ArchiveType::Dir => {
            log::debug!("load file");

            archive::dir::load_file(archive_path, page_pos).unwrap()
        }

        _ => {
            todo!()
        }
    };

    resize::resize_bytes(bytes.as_slice(), buffer, screen_size, window_size, filter);
}

mod test {

    #[test]
    fn _push_front() {
        use super::*;

        let mut a = vec![4, 5, 6];
        push_front(&mut a, [1, 2, 3].as_slice());

        assert_eq!(a.as_slice(), [1, 2, 3, 4, 5, 6].as_slice());
    }
}
