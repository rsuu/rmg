use crate::{
    archive::{self, ArchiveType},
    color::rgba::TransRgba,
    img::{resize, size::Size},
    reader::{
        keymap::Map,
        view::{Buffer, Img, Page},
        window::Canvas,
    },
    utils::err::{MyErr, Res},
    TIMER,
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

#[derive(Debug)]
pub struct Scroll {
    pub buffer: Buffer,
    pub max_ram: usize,

    pub head: usize, // =0
    pub tail: usize, // =0

    pub rng: usize, //

    pub page_list: Vec<Page>,
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

    pub len: usize,
    pub page_load_list: Vec<usize>,
}

impl Scroll {
    pub fn page_list_len(&mut self) -> usize {
        let mut res = 0;

        self.page_load_list.clear();

        for (idx, page) in self.page_list.iter().enumerate() {
            if page.len() > 0 {
                res += page.len();
                self.page_load_list.push(idx);
            } else {
            }
        }

        res
    }

    /// init
    pub fn init(&mut self) {
        // only works when page count > 0
        if self.page_end > 0 {
            let mut len = 0;

            len += self.load_next();

            'l1: while len <= self.max_ram * 2 && self.tail + 1 < self.page_end {
                self.tail += 1;
                len += self.load_next();
            }
        } else {
            panic!()
        }
    }

    #[inline(always)]
    pub fn flush(&mut self, canvas: &mut Canvas, arc_state: &Arc<RwLock<State>>) {
        unsafe {
            if TIMER >= 60 {
                TIMER = 0;
            } else {
                TIMER += 1;
            }
        }

        if *arc_state.read().unwrap() == State::Nothing {
            // update

            unsafe {
                if TIMER == 0 {
                    // TODO: gif next frame
                    for idx in self.page_load_list.iter() {
                        self.page_list[*idx].to_next_frame();
                    }
                } else {
                }
            }

            self.buffer.clear();
            self.len = self.page_list_len();

            for idx in self.page_load_list.iter() {
                self.buffer.flush(&self.page_list[*idx]);
            }
        } else {
        }

        canvas.flush(&self.buffer.data[self.rng..self.rng + self.max_ram]);
    }

    #[inline(always)]
    pub fn load_next(&mut self) -> usize {
        if let Some(img) = load_img(
            self.archive_type,
            self.archive_path.as_path(),
            self.page_list[self.tail].pos,
            self.screen_size,
            self.window_size,
            self.filter,
        ) {
            self.page_list[self.tail].flush(img)
        } else {
            panic!()
        }
    }

    /// goto next page
    // HACK: async version
    #[inline]
    pub fn move_down(&mut self, arc_page: &Arc<RwLock<Page>>, arc_state: &Arc<RwLock<State>>) {
        debug!(
            "
state: {:?}
start: {}
end: {}
len: {}
head: {}
tail: {}
",
            *arc_state.read().unwrap(),
            self.rng,
            self.end(),
            self.len,
            self.head,
            self.tail,
        );

        // scrolling viewer
        if self.len >= self.end() + self.y_step {
            self.rng += self.y_step;
        } else if self.len >= self.end() {
            self.rng += self.len - self.end();
        } else {
        }

        // change the state
        self.mode = Map::Down;

        info!("move_down()");

        // try to load next page
        if self.end() >= self.len / 8
            && self.tail + 1 < self.page_end
            && *arc_state.read().unwrap() == State::Nothing
        {
            self.tail += 1;

            let page = arc_page.clone();
            let state = arc_state.clone();
            *state.write().unwrap() = State::NextReady;

            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let page_pos = self.page_list[self.tail].pos;
            let screen_size = self.screen_size;
            let window_size = self.window_size;
            let filter = self.filter;
            let mut tail = self.page_list[self.tail].clone();

            tokio::spawn(async move {
                if let Some(img) = load_img(
                    archive_type,
                    archive_path.as_path(),
                    page_pos,
                    screen_size,
                    window_size,
                    filter,
                ) {
                    tail.flush(img);
                    *page.write().unwrap() = tail;
                    *state.write().unwrap() = State::NextDone;

                    debug!("DONE");
                }
            });
        }

        // load next
        if *arc_state.read().unwrap() == State::NextDone {
            self.page_list[self.tail] = arc_page.read().unwrap().clone();

            debug!("state == {:?}", arc_state.read().unwrap());
            debug!("load next");

            *arc_state.write().unwrap() = State::Nothing;
        } else {
        }

        let head_len = self.page_list[self.head].len();

        // try to free up the memory
        if *arc_state.write().unwrap() == State::Nothing
            && self.len >= self.max_ram * 8 + head_len
            && self.head + 1 < self.tail
            && self.rng > head_len
        {
            self.rng -= self.page_list[self.head].len();
            self.page_list[self.head].clear();
            self.head += 1;
        }
    }

    /// goto prev page
    #[inline]
    pub fn move_up(&mut self, arc_page: &Arc<RwLock<Page>>, arc_state: &Arc<RwLock<State>>) {
        debug!(
            "
state: {:?}
start: {}
end: {}
len: {}
head: {}
tail: {}
",
            *arc_state.read().unwrap(),
            self.rng,
            self.end(),
            self.len,
            self.head,
            self.tail,
        );

        if self.rng >= self.y_step {
            self.rng -= self.y_step;
        } else if self.rng >= 0 {
            self.rng = 0;
        } else {
        }

        self.mode = Map::Up;

        info!("move_up()");

        // try to load prev page
        if self.head >= 1
            && (self.rng <= self.max_ram * 2)
            && (*arc_state.read().unwrap() == State::Nothing
                || *arc_state.read().unwrap() == State::NextDone)
        {
            self.head -= 1;

            let page = arc_page.clone();
            let state = arc_state.clone();
            *state.write().unwrap() = State::PrevReady;

            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let page_pos = self.page_list[self.head].pos;
            let screen_size = self.screen_size;
            let window_size = self.window_size;
            let filter = self.filter;
            let mut head = self.page_list[self.head].clone();

            tokio::spawn(async move {
                if let Some(img) = load_img(
                    archive_type,
                    archive_path.as_path(),
                    page_pos,
                    screen_size,
                    window_size,
                    filter,
                ) {
                    head.flush(img);
                    *page.write().unwrap() = head;
                    *state.write().unwrap() = State::PrevDone;

                    debug!("DONE");
                }
            });
        } else {
        }

        // load prev
        if *arc_state.read().unwrap() == State::PrevDone {
            self.page_list[self.head] = arc_page.read().unwrap().clone();
            self.rng += self.page_list[self.head].len();

            *arc_state.write().unwrap() = State::Nothing;

            debug!("state == {:?}", arc_state.read().unwrap());
            debug!("load prev");
        } else {
        }

        let tail_len = self.page_list[self.tail].len();

        // HACK: try to free up the memory
        if *arc_state.write().unwrap() == State::Nothing
            && self.len >= self.end() + tail_len
            && self.len >= self.max_ram * 8 + tail_len
            && self.tail > self.head + 1
        {
            self.page_list[self.tail].clear();
            self.tail -= 1;

            debug!("move_up: free()");
        }
    }

    pub fn move_left(&mut self) {
        // HACK: overflow
        // ??? How it works
        if self.len > self.end() + self.x_step {
            self.rng += self.x_step;

            debug!("start: {}", self.rng);
            debug!("end: {}", self.end());
        } else {
        }

        self.mode = Map::Left;
    }

    ///
    pub fn move_right(&mut self) {
        // HACK: overflow
        if self.rng >= self.x_step {
            self.rng -= self.x_step;
        } else {
        }

        self.mode = Map::Right;
    }

    pub fn end(&self) -> usize {
        self.rng + self.max_ram
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
#[inline(always)]
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
