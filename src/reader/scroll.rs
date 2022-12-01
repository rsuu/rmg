use crate::{
    archive::{self, ArchiveType},
    color::rgba::TransRgba,
    img::{resize, size::Size},
    reader::{
        keymap::Map,
        view::{Img, Page},
        window::Canvas,
    },
    utils::err::{MyErr, Res},
};
use fir;
use log::{debug, info};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use tokio;

use super::view::{ImgBit, ImgGif};

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

//use crate::reader::view::Page;

#[derive(Debug, Clone)]
pub struct Scroll {
    pub buffer: Vec<u32>,
    pub max_ram: usize,

    pub head: usize, // =0
    pub tail: usize, // =0

    pub start: usize,
    pub end: usize,

    pub page_list: Vec<Page>,
    //pub page_list: Vec<Page>,
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

impl Scroll {
    /// init
    pub fn init(&mut self) {
        // only works when page count >= 1
        if self.page_end >= 1 {
            self.load_next();

            'l1: while self.buffer.len() <= self.max_ram * 2 && self.tail + 1 < self.page_end {
                self.tail += 1;
                self.load_next();
            }
        } else {
            panic!()
        }
    }

    #[inline(always)]
    pub fn flush(&self, canvas: &mut Canvas) {
        canvas.flush(&self.buffer[self.start..self.end]);

        // debug!("self.buffer.len() == {}", self.buffer.len());
    }

    pub fn load_next(&mut self) {
        info!("load next");

        let pos = self.page_list[self.tail].pos;

        if let Some(img) = load_img(
            self.archive_type,
            self.archive_path.as_path(),
            pos,
            self.screen_size,
            self.window_size,
            self.filter,
        ) {
            self.buffer.extend_from_slice(img.data().unwrap());
            self.page_list[self.tail].len = img.len();
        } else {
            todo!()
        }
    }

    /// goto next page
    // HACK: async version
    #[inline]
    pub fn move_down(
        &mut self,
        color_buffer_arc: &Arc<RwLock<Vec<u32>>>,
        state_arc: &Arc<RwLock<State>>,
    ) {
        debug!(
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
            self.buffer.len(),
            self.head,
            self.tail,
        );

        let cut_len = self.page_list[self.head].len;

        // try to load next page
        if self.end >= self.buffer.len() / 8
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
                if let Some(img) = load_img(
                    archive_type,
                    archive_path.as_path(),
                    page_pos,
                    screen_size,
                    window_size,
                    filter,
                ) {
                    color_buf
                        .write()
                        .unwrap()
                        .extend_from_slice(img.data().unwrap());

                    *state.write().unwrap() = State::NextDone;

                    debug!("DONE");
                }
            });

            drop(join);
        }

        // load next
        if *state_arc.read().unwrap() == State::NextDone {
            self.page_list[self.tail].len = color_buffer_arc.read().unwrap().len();
            self.buffer
                .extend_from_slice(color_buffer_arc.read().unwrap().as_slice());

            color_buffer_arc.write().unwrap().clear();

            debug!("state == {:?}", state_arc.read().unwrap());
            debug!("load next");

            *state_arc.write().unwrap() = State::Nothing;
        } else {
        }

        // try to free up the memory
        if *state_arc.write().unwrap() == State::Nothing
            && self.start > cut_len
            && self.buffer.len() >= self.max_ram * 8 + cut_len
            && self.head + 1 < self.tail
        {
            self.head += 1;
            free_head(&mut self.buffer, cut_len);

            self.start -= cut_len;
            self.end -= cut_len;
        }

        // scrolling viewer
        if self.buffer.len() >= self.end + self.y_step {
            self.start += self.y_step;
            self.end += self.y_step;
        } else if self.buffer.len() >= self.end {
            self.start += self.buffer.len() - self.end;
            self.end += self.buffer.len() - self.end;
        } else {
        }

        // change the state
        self.mode = Map::Down;

        info!("move_down()");
    }

    /// goto prev page
    #[inline]
    pub fn move_up(
        &mut self,
        color_buffer_arc: &Arc<RwLock<Vec<u32>>>,
        state_arc: &Arc<RwLock<State>>,
    ) {
        debug!(
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
            self.buffer.len(),
            self.head,
            self.tail,
        );

        let cut_len = self.page_list[self.tail].len;

        // try to load prev page
        if self.head >= 1
            && (self.start <= self.max_ram * 2)
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
                if let Some(img) = load_img(
                    archive_type,
                    archive_path.as_path(),
                    page_pos,
                    screen_size,
                    window_size,
                    filter,
                ) {
                    color_buf
                        .write()
                        .unwrap()
                        .extend_from_slice(img.data().unwrap());

                    *state.write().unwrap() = State::PrevDone;

                    debug!("DONE");
                }
            });

            drop(join);
        } else {
        }

        // load prev
        if *state_arc.read().unwrap() == State::PrevDone {
            push_front(
                &mut self.buffer,
                color_buffer_arc.read().unwrap().as_slice(),
            );

            let len = color_buffer_arc.read().unwrap().len();

            self.start += len;
            self.end += len;

            color_buffer_arc.write().unwrap().clear();
            *state_arc.write().unwrap() = State::Nothing;

            debug!("state == {:?}", state_arc.read().unwrap());
            debug!("load prev");
        } else {
        }

        // HACK: try to free up the memory
        if *state_arc.write().unwrap() == State::Nothing
            && self.buffer.len() >= self.end + cut_len
            && self.buffer.len() >= self.max_ram * 8 + cut_len
            && self.tail > self.head + 1
        {
            self.tail -= 1;
            free_tail(&mut self.buffer, cut_len);

            debug!("move_up: free()");
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

        info!("move_up()");
    }

    pub fn move_left(&mut self) {
        // HACK: overflow
        // ??? How it works
        if self.buffer.len() > self.end + self.x_step {
            self.start += self.x_step;
            self.end += self.x_step;

            debug!("start: {}", self.start);
            debug!("end: {}", self.end);
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
        self.buffer.len() <= self.max_ram * LOAD_MAX
    }

    ///
    pub fn need_pad_head(&self) -> bool {
        self.buffer.len() <= self.max_ram * LOAD_MAX
    }

    ///
    pub fn at_block_head(&self) -> bool {
        self.start == 0
    }

    ///
    pub fn at_block_tail(&self) -> bool {
        self.end == self.buffer.len()
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

#[inline(always)]
pub fn push_front<T>(vec: &mut Vec<T>, slice: &[T]) {
    let amt = slice.len(); // [1, 2, 3]
    let len = vec.len(); // [4, 5, 6]

    vec.reserve(amt);

    unsafe {
        std::ptr::copy(vec.as_ptr(), vec.as_mut_ptr().offset((amt) as isize), len);
        std::ptr::copy(slice.as_ptr(), vec.as_mut_ptr(), amt);

        vec.set_len(len + amt);
    }
}

#[inline(always)]
pub fn free_head<T>(buffer: &mut Vec<T>, range: usize)
where
    T: Sized + Clone,
{
    buffer.drain(..range);
}

#[inline(always)]
pub fn free_tail<T>(buffer: &mut Vec<T>, range: usize)
where
    T: Sized,
{
    buffer.truncate(buffer.len() - range);
}

///
#[inline]
pub fn load_img(
    archive_type: ArchiveType,
    archive_path: &Path,
    page_pos: usize,
    screen_size: Size<u32>,
    window_size: Size<u32>,
    filter: fir::FilterType,
) -> Option<Img> {
    debug!("archive_type == {:?}", archive_type);

    let bytes = match archive_type {
        ArchiveType::Tar => {
            debug!("ex_tar()");

            archive::tar::load_file(archive_path, page_pos).unwrap()
        }

        ArchiveType::Zip => {
            debug!("ex_zip()");

            archive::zip::load_file(archive_path, page_pos).unwrap()
        }

        ArchiveType::Dir => {
            debug!("load file");

            archive::dir::load_file(archive_path, page_pos).unwrap()
        }

        _ => {
            todo!()
        }
    };

    let mut ty = img_type(&bytes);
    let mut temp = Vec::new();

    match ty {
        Img::Bit(ref mut img) => {
            resize::resize_bytes(
                &mut temp,
                bytes.as_slice(),
                screen_size,
                window_size,
                &filter,
            );

            for f in (0..temp.len()).step_by(4) {
                img.data
                    .push(TransRgba::argb_to_u32(&temp[f..f + 4].try_into().unwrap()));
            }

            return Some(ty);
        }
        Img::Gif(gif) => {
            todo!()
        }
        Img::Unknown => None,
    }
}

pub fn img_type(buffer: &[u8]) -> Img {
    match infer::get(buffer) {
        Some(ty) => match ty.extension() {
            "jpg" | "png" | "heic" => Img::Bit(ImgBit::new()),
            "gif" => {
                todo!();
            }
            _ => Img::Unknown,
        },
        None => Img::Unknown,
    }
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
