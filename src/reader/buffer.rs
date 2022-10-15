use crate::{
    archive,
    color::rgb::TransRgb,
    img::{
        resize::{self},
        size::Size,
    },
    reader::{keymap::Map, window::Canvas},
    utils::types::ArchiveType,
};
use log;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use tokio;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    NextReady,
    NextDone,
    NextLoad,

    PrevReady,
    PrevDone,
    PrevLoad,
}

#[derive(Debug, Clone)]
pub struct PageInfo {
    pub path: PathBuf, // drop
    pub name: String,  // name of image
    pub len: usize,    // drop
    pub pos: usize,    // index of the image in the archive file
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

    pub x_step: usize, // move_down AND move_up
    pub y_step: usize, // move_left AND move_right

    pub mode: Map,

    pub window_size: Size<u32>,
    pub screen_size: Size<u32>,
    pub window_position: (i32, i32),

    pub range_start: usize,
    pub range_end: usize,
}

impl Buffer {
    ///
    pub fn init(&mut self) {
        if self.page_list.len() > 1 {
            self.range_start += 1;

            'l1: while self.bytes.len() < self.max_bytes * 2 && self.range_end + 1 < self.page_end {
                log::info!("load next");
                self.range_end += 1;

                let page_pos = self.page_list[self.range_end].pos;

                let mut img_buf = Vec::new();

                get_rgb_buffer(
                    &mut img_buf,
                    self.archive_type,
                    self.archive_path.as_path(),
                    page_pos,
                    self.screen_size,
                    self.window_size,
                );

                self.bytes.extend_from_slice(img_buf.as_slice());
                self.page_list[self.range_end].len = img_buf.len();
            }
        } else {
            panic!()
        }
    }

    /// goto next page
    pub fn move_down(
        &mut self,
        color_buffer_arc: &Arc<RwLock<Vec<u32>>>,
        state_arc: &Arc<RwLock<State>>,
    ) {
        // HACK: async version
        log::debug!("start: {}", self.range_start);

        let cut_len = self.page_list[self.range_start].len;

        // load image
        if (self.at_block_tail() || self.need_pad_next())
            && self.range_end + 1 < self.page_end
            && (*state_arc.read().unwrap() == State::NextLoad
                || *state_arc.read().unwrap() == State::PrevLoad)
        {
            self.range_end += 1;

            // this block will only works once before `state == State::Start`
            let color_buf = color_buffer_arc.clone();
            let state = state_arc.clone();
            *state.write().unwrap() = State::NextReady;

            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let page_pos = self.page_list[self.range_end].pos;
            let screen_size = self.screen_size;
            let window_size = self.window_size;

            let join = tokio::spawn(async move {
                let mut img_selffer = Vec::new();

                get_rgb_buffer(
                    &mut img_selffer,
                    archive_type,
                    archive_path.as_path(),
                    page_pos,
                    screen_size,
                    window_size,
                );

                color_buf
                    .write()
                    .unwrap()
                    .extend_from_slice(img_selffer.as_slice());

                *state.write().unwrap() = State::NextDone;

                log::debug!("DONE");
            }); // NOTE: DO NOT use `join.await`

            drop(join);
        }

        // load next
        if *state_arc.read().unwrap() == State::NextDone && self.not_page_tail() {
            self.page_list[self.range_end].len = color_buffer_arc.read().unwrap().len();
            self.bytes
                .extend_from_slice(color_buffer_arc.read().unwrap().as_slice());

            color_buffer_arc.write().unwrap().clear();

            log::debug!("state == {:?}", state_arc.read().unwrap());
            log::debug!("load next");

            *state_arc.write().unwrap() = State::NextLoad;

            // try to free up the memory
            while self.start > cut_len
                && self.bytes.len() >= self.max_bytes * 4 + cut_len
                && self.range_start < self.range_end
            {
                self.range_start += 1;
                free_head(&mut self.bytes, cut_len);

                self.start -= cut_len;
                self.end -= cut_len;
            }
        } else {
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
        let cut_len = self.page_list[self.range_end].len;

        // load image
        if (self.at_block_head() || self.need_pad_prev())
            && self.range_start > 1
            && (*state_arc.read().unwrap() == State::PrevLoad
                || *state_arc.read().unwrap() == State::NextLoad)
        {
            self.range_start -= 1;

            let color_buf = color_buffer_arc.clone();
            let state = state_arc.clone();
            *state.write().unwrap() = State::PrevReady;

            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let page_pos = self.page_list[self.range_start].pos;
            let screen_size = self.screen_size;
            let window_size = self.window_size;

            let join = tokio::spawn(async move {
                let mut img_selffer = Vec::new();

                get_rgb_buffer(
                    &mut img_selffer,
                    archive_type,
                    archive_path.as_path(),
                    page_pos,
                    screen_size,
                    window_size,
                );

                color_buf
                    .write()
                    .unwrap()
                    .extend_from_slice(img_selffer.as_slice());

                *state.write().unwrap() = State::PrevDone;

                log::debug!("DONE");
            });

            // WARN: DO NOT use `join.await`
            drop(join);

            log::debug!("{}", self.range_start);
        } else {
        }

        // load prev
        if *state_arc.read().unwrap() == State::PrevDone {
            push_front(&mut self.bytes, color_buffer_arc.read().unwrap().as_slice());

            let len = color_buffer_arc.read().unwrap().len();

            self.start += len;
            self.end += len;

            color_buffer_arc.write().unwrap().clear();
            *state_arc.write().unwrap() = State::PrevLoad;

            log::debug!("state == {:?}", state_arc.read().unwrap());
            log::debug!("load prev");

            // HACK: try to free up the memory
            while self.bytes.len() >= self.end + cut_len
                && self.bytes.len() >= self.max_bytes * 2 + cut_len
                && self.range_start < self.range_end
                && self.range_end > 1
            {
                self.range_end -= 1;

                free_tail(&mut self.bytes, cut_len);

                log::debug!("move_up: free()");
            }
        } else {
        }

        if self.start >= self.y_step {
            self.start -= self.y_step;
            self.end -= self.y_step;
        // } else if self.start >= 0 {
        //     let s = self.start;
        //     self.start -= s;
        //     self.end -= s;
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

    pub fn move_right(&mut self) {
        // HACK: overflow
        if self.start >= self.x_step {
            self.start -= self.x_step;
            self.end -= self.x_step;
        } else {
        }

        self.mode = Map::Right;
    }

    #[inline(always)]
    pub fn flush(&self, canvas: &mut Canvas) {
        canvas.flush(&self.bytes[self.start..self.end]);

        //log::debug!("self.bytes.len() == {}", self.bytes.len());
    }

    pub fn need_pad_next(&self) -> bool {
        self.bytes.len() <= self.max_bytes * 6
    }

    pub fn need_pad_prev(&self) -> bool {
        self.bytes.len() <= self.max_bytes * 8
    }

    pub fn at_block_head(&self) -> bool {
        self.start == 0
    }

    pub fn at_block_tail(&self) -> bool {
        self.end == self.bytes.len()
    }

    pub fn not_page_tail(&self) -> bool {
        self.range_end < self.page_end
    }

    pub fn to_prev_page(&mut self) {
        self.range_start -= 1;
    }
}

#[inline]
pub fn push_front<T>(vec: &mut Vec<T>, slice: &[T])
where
    T: ?Copy + Clone,
{
    unsafe {
        let len = vec.len();
        let amt = slice.len();

        vec.reserve(amt);

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

pub fn get_rgb_buffer(
    buffer: &mut Vec<u32>,
    archive_type: ArchiveType,
    archive_path: &Path,
    page_pos: usize,
    screen_size: Size<u32>,
    window_size: Size<u32>,
) {
    let mut img = Vec::new();

    resize_img(
        &mut img,
        archive_type,
        archive_path,
        page_pos,
        screen_size,
        window_size,
    );

    for f in (0..img.len()).step_by(3) {
        buffer.push(TransRgb::rgb_to_u32(&img[f..f + 3].try_into().unwrap()));
    }
}

pub fn resize_img(
    buffer: &mut Vec<u8>,
    archive_type: ArchiveType,
    archive_path: &Path,
    page_pos: usize,
    screen_size: Size<u32>,
    window_size: Size<u32>,
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

    resize::resize_bytes(bytes.as_slice(), buffer, screen_size, window_size);
}

mod test {
    

    #[test]
    fn _push_front() {
        let mut a = vec![4, 5, 6];
        push_front(&mut a, [1, 2, 3].as_slice());

        assert_eq!(a.as_slice(), [1, 2, 3, 4, 5, 6].as_slice());
    }
}
