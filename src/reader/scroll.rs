use crate::{
    archive::{self, ArchiveType},
    color::rgba::TransRgba,
    img::{resize, size::Size},
    reader::{
        keymap::Map,
        view::{Buffer, ImgType, Page, ViewMode},
        window::Canvas,
    },
    utils::{
        err::{MyErr, Res},
        file,
    },
    TIMER,
};
use fir;
use log::{debug, info};
use pollster::block_on;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread::spawn,
};

const LOAD_MAX: usize = 2;

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
pub struct Render {
    pub buffer: Buffer,
    pub buffer_max: usize,
    pub mem_limit: usize,

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

    pub view_mode: ViewMode,
}

impl Render {
    /// init
    pub fn init(&mut self) {
        let mut len = 0;

        if self.page_end >= 2 {
            len += self.load_next();

            'l1: while len <= self.mem_limit && self.tail + 1 < self.page_end {
                self.tail += 1;
                len += self.load_next();
            }
        } else if self.page_end >= 1 {
            len += self.load_next();
        } else {
        }

        // image is smaller than buffer
        if len <= self.buffer_max {
            self.view_mode = ViewMode::Image;

            self.buffer.flush(&self.page_list[0]);
            self.buffer
                .data
                .extend_from_slice(&vec![0; self.buffer_max - self.buffer.data.len()]);
        } else {
        }

        debug!("*** INIT ***");
        debug!("    len = {}", len);
    }

    pub fn load_next(&mut self) -> usize {
        let mut tail = &mut self.page_list[self.tail];
        let pos = tail.pos;

        load_img(
            &mut tail,
            self.archive_type,
            self.archive_path.as_path(),
            pos,
            self.screen_size,
            self.window_size,
            self.filter,
        );

        tail.len()
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

        if *arc_state.read().unwrap() == State::Nothing
            || *arc_state.read().unwrap() == State::NextReady
            || *arc_state.read().unwrap() == State::PrevReady
        {
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

        canvas.flush(&self.buffer.data[self.rng..self.rng + self.buffer_max]);
    }

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

        // scrolling
        self.rng += if self.len >= self.end() + self.y_step {
            self.y_step
        } else if self.len >= self.end() {
            self.len - self.end()
        } else {
            0
        };

        // change the state
        self.mode = Map::Down;

        info!("move_down()");

        // try to load next page
        if self.end() >= self.len / 8
            && self.tail + 1 < self.page_end
            && *arc_state.read().unwrap() == State::Nothing
            || *arc_state.read().unwrap() == State::PrevDone
        {
            self.tail += 1;

            let state = arc_state.clone();
            *state.write().unwrap() = State::NextReady;

            let page = arc_page.clone();

            let mut tail = self.page_list[self.tail].clone();
            //tail.loading();

            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let page_pos = self.page_list[self.tail].pos;
            let screen_size = self.screen_size;
            let window_size = self.window_size;
            let filter = self.filter;

            spawn(move || {
                block_on(async {
                    load_img(
                        &mut tail,
                        archive_type,
                        archive_path.as_path(),
                        page_pos,
                        screen_size,
                        window_size,
                        filter,
                    );

                    *page.write().unwrap() = tail;
                    *state.write().unwrap() = State::NextDone;

                    debug!("DONE");
                })
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
            && self.len >= self.mem_limit + head_len
            && self.head + 1 < self.tail
            && self.rng > head_len
        {
            self.rng -= self.page_list[self.head].len();
            self.page_list[self.head].clear();
            self.head += 1;

            debug!("*** FREE ***")
        } else {
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
        } else {
            self.rng = 0;
        }

        self.mode = Map::Up;

        info!("move_up()");

        // try to load prev page
        if self.head >= 1
            && (self.rng <= self.mem_limit)
            && (*arc_state.read().unwrap() == State::Nothing
                || *arc_state.read().unwrap() == State::NextDone)
        {
            self.head -= 1;

            let state = arc_state.clone();
            *state.write().unwrap() = State::PrevReady;

            let page = arc_page.clone();

            let mut head = self.page_list[self.head].clone();
            //head.loading();

            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let page_pos = self.page_list[self.head].pos;
            let screen_size = self.screen_size;
            let window_size = self.window_size;
            let filter = self.filter;

            spawn(move || {
                block_on(async {
                    load_img(
                        &mut head,
                        archive_type,
                        archive_path.as_path(),
                        page_pos,
                        screen_size,
                        window_size,
                        filter,
                    );

                    *page.write().unwrap() = head;
                    *state.write().unwrap() = State::PrevDone;

                    debug!("DONE");
                })
            });

        // tokio::spawn(async move {
        //     ...
        // });
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

        // try to free up the memory
        if *arc_state.write().unwrap() == State::Nothing
            && self.len >= self.end() + tail_len
            && self.len >= self.mem_limit + tail_len
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
        self.rng + self.buffer_max
    }
}

// #[inline(always)]
// pub fn push_front<T>(vec: &mut Vec<T>, slice: &[T]) {
//     let amt = slice.len(); // [1, 2, 3]
//     let len = vec.len(); // [4, 5, 6]
//
//     vec.reserve(amt);
//
//     unsafe {
//         std::ptr::copy(vec.as_ptr(), vec.as_mut_ptr().offset((amt) as isize), len);
//         std::ptr::copy(slice.as_ptr(), vec.as_mut_ptr(), amt);
//
//         vec.set_len(len + amt);
//     }
// }
//
// #[inline(always)]
// pub fn free_head<T>(buffer: &mut Vec<T>, range: usize)
// where
//     T: Sized + Clone,
// {
//     buffer.drain(..range);
// }
//
// #[inline(always)]
// pub fn free_tail<T>(buffer: &mut Vec<T>, range: usize)
// where
//     T: Sized,
// {
//     buffer.truncate(buffer.len() - range);
// }

///
#[inline(always)]
pub fn load_img(
    page: &mut Page,
    archive_type: ArchiveType,
    archive_path: &Path,
    page_pos: usize,
    screen_size: Size<u32>,
    window_size: Size<u32>,
    filter: fir::FilterType,
) {
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
            panic!()
        }
    };

    debug!("    len = {}", bytes.len());

    let opt_page: Option<Page> = match infer::get(&bytes) {
        Some(ty) => match ty.extension() {
            "jpg" | "png" | "heic" | "heif" | "avif" => Some(Page::new_bit()),
            "gif" => {
                todo!();
            }
            _ => panic!(),
        },
        None => {
            if file::is_aseprite(&bytes) {
                Some(Page::new_anim())
            } else {
                None
            }
        }
    };

    if let Some(mut img) = opt_page {
        let mut tmp = Vec::new();

        match img.ty {
            ImgType::Bit => {
                let size =
                    resize::resize_bytes(&mut tmp, &bytes, screen_size, window_size, &filter);

                for f in (0..tmp.len()).step_by(4) {
                    img.data[0].push(TransRgba::argb_to_u32(&tmp[f..f + 4].try_into().unwrap()));
                }

                debug!("load img");
                debug!("    len = {}", img.len());

                page.data = img.data;
                page.resize = img.resize;
            }

            ImgType::Anim => {
                todo!()
            }

            _ => todo!(),
        }
    } else {
    }
}
