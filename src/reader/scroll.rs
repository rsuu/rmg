use crate::{
    archive::{self, ArchiveType},
    color::rgba::TransRgba,
    img::{
        ase, gif, heic, resize,
        size::{MetaSize, Size, TMetaSize},
    },
    reader::{
        keymap::Map,
        view::{Buffer, ImgFormat, ImgType, Page, ViewMode},
        window::Canvas,
    },
    utils::{
        err::{MyErr, Res},
        file,
        traits::AutoLog,
    },
    TIMER,
};
use fir::{self, FilterType};
use log::{debug, info};
use std::{
    mem,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread::spawn,
};

const LOAD_MAX: usize = 2;

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

    pub page_number: u32,
    pub page_loading: Vec<u32>,
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

        // image.len() < buffer.len()
        if len <= self.buffer_max {
            self.view_mode = ViewMode::Image;

            self.buffer.flush(&self.page_list[0].data());
            self.buffer
                .data
                .extend_from_slice(&vec![0; self.buffer_max - self.buffer.data.len()]);
        } else {
        }

        debug!("*** INIT ***");
        debug!("    len = {}", len);
    }

    pub fn load_next(&mut self) -> usize {
        let tail = &mut self.page_list[self.tail];
        let pos = tail.pos;

        let (ty, mut buffer, format) =
            load_page(self.archive_type, self.archive_path.as_path(), pos);
        let meta = open_img(format, &mut buffer, self.screen_size, self.window_size).unwrap();

        tail.ty = ty;
        tail.resize = meta.fix;

        resize_page(tail, &mut buffer, &meta, &self.filter);

        tail.is_ready = true;

        tail.len()
    }

    #[inline(always)]
    pub fn flush(&mut self, canvas: &mut Canvas, arc_state: &Arc<RwLock<State>>) {
        unsafe {
            if TIMER == 0 {
                // TODO: gif next frame
                for idx in self.page_load_list.iter() {
                    self.page_list[*idx].to_next_frame();
                }
            } else if TIMER >= 60 {
                TIMER = 0;
            } else {
                TIMER += 1;
            }
        }

        self.buffer.clear();
        self.len = self.page_list_len();

        for idx in self.page_load_list.iter() {
            if self.page_list[*idx].is_ready {
                self.buffer.flush(self.page_list[*idx].data());
            } else {
                self.buffer
                    .data
                    .extend_from_slice(&self.page_loading[0..self.buffer_max * 4]);
            }
        }

        canvas.flush(&self.buffer.data[self.rng..self.rng + self.buffer_max]);

        // dbg!("self.page_number = {}", self.page_number);
    }

    #[inline]
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
    #[inline]
    pub fn move_down(&mut self, arc_page: &Arc<RwLock<Page>>, arc_state: &Arc<RwLock<State>>) {
        info!("move_down()");

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

        self.mode = Map::Down;

        // scrolling
        self.rng += if self.len >= self.end() + self.y_step {
            self.y_step
        } else if self.len >= self.end() {
            self.len - self.end()
        } else {
            0
        };

        // try to load next page
        if self.end() <= self.mem_limit
            && self.tail + 1 < self.page_end
            && (*arc_state.read().unwrap() == State::Nothing
                || *arc_state.read().unwrap() == State::PrevDone)
        {
            let state = arc_state.clone();
            *state.write().unwrap() = State::NextReady;

            let page = arc_page.clone();

            self.tail += 1;

            let idx = self.tail;
            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let screen_size = self.screen_size;
            let window_size = self.window_size;
            let filter = self.filter;

            let mut tail = self.page_list[idx].clone();
            let pos = tail.pos;
            tail.is_ready = false;

            info!("load next");
            spawn(move || {
                let (ty, mut buffer, format) = load_page(archive_type, archive_path.as_path(), pos);
                let meta = open_img(format, &mut buffer, screen_size, window_size).unwrap();

                tail.ty = ty;
                tail.resize = meta.fix;

                resize_page(&mut tail, &mut buffer, &meta, &filter);

                *page.write().unwrap() = tail;
                *state.write().unwrap() = State::NextDone;

                debug!("*** DONE ***");
            });
        }

        // load next
        if *arc_state.read().unwrap() == State::NextDone {
            debug!("state == {:?}", arc_state.read().unwrap());

            mem::swap(
                &mut self.page_list[self.tail],
                &mut *arc_page.write().unwrap(),
            );

            self.page_list[self.tail].is_ready = true;

            *arc_state.write().unwrap() = State::Nothing;
        } else {
        }

        let head_len = self.page_list[self.head].len();

        // try to free up the memory
        if *arc_state.read().unwrap() == State::Nothing
            && self.len >= self.mem_limit + head_len
            && self.head + 1 < self.tail
            && self.rng > head_len
        {
            self.page_list[self.head].is_ready = false;
            self.rng -= self.page_list[self.head].len();
            self.page_list[self.head].free();
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

        self.rng -= if self.rng >= self.y_step {
            self.y_step
        } else {
            0
        };

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

            let idx = self.head;
            let archive_type = self.archive_type;
            let archive_path = self.archive_path.clone();
            let page_pos = self.page_list[self.head].pos;
            let screen_size = self.screen_size;
            let window_size = self.window_size;
            let filter = self.filter;

            let mut head = self.page_list[idx].clone();
            head.is_ready = false;
            let pos = head.pos;

            "load prev"._info();

            spawn(move || {
                let (ty, mut buffer, format) = load_page(archive_type, archive_path.as_path(), pos);
                let meta = open_img(format, &mut buffer, screen_size, window_size).unwrap();

                head.ty = ty;
                head.resize = meta.fix;

                resize_page(&mut head, &mut buffer, &meta, &filter);

                *page.write().unwrap() = head;
                *state.write().unwrap() = State::PrevDone;

                "*** DONE ***"._info();
            });
        } else {
        }

        // load prev
        if *arc_state.read().unwrap() == State::PrevDone {
            mem::swap(
                &mut self.page_list[self.head],
                &mut *arc_page.write().unwrap(),
            );
            self.page_list[self.head].is_ready = true;
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
            self.page_list[self.tail].is_ready = false;
            self.page_list[self.tail].free();
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

///
#[inline(always)]
pub fn load_page(
    archive_type: ArchiveType,
    archive_path: &Path,
    page_pos: usize,
) -> (ImgType, Vec<Vec<u8>>, ImgFormat) {
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

    let bytes = vec![bytes];

    debug!("    len = {}", bytes.len());

    let mut format = ImgFormat::Unknown;

    if let Some(ty) = infer::get(&bytes[0]) {
        format = ImgFormat::from(ty.extension());
    } else if file::is_aseprite(&bytes[0]) {
        format = ImgFormat::Aseprite;
    } else {
    }

    match format {
        ImgFormat::Jpg | ImgFormat::Png | ImgFormat::Heic | ImgFormat::Avif => {
            (ImgType::Bit, bytes, format)
        }

        ImgFormat::Aseprite | ImgFormat::Gif => (ImgType::Anim, bytes, format),

        _ => {
            todo!()
        }
    }
}

#[inline(always)]
pub fn open_img(
    format: ImgFormat,
    bytes: &mut Vec<Vec<u8>>,
    screen_size: Size<u32>,
    window_size: Size<u32>,
) -> Res<MetaSize<u32>> {
    let mut meta = MetaSize::<u32>::new(
        screen_size.width,
        screen_size.height,
        window_size.width,
        window_size.height,
        0,
        0,
    );

    match format {
        ImgFormat::Jpg | ImgFormat::Png => {
            let img = image::load_from_memory(&bytes[0])?;
            meta.image.width = img.width();
            meta.image.height = img.height();
            meta.resize();

            mem::swap(bytes, &mut vec![img.to_rgba8().to_vec()]);

            Ok(meta)
        }

        ImgFormat::Heic | ImgFormat::Avif => {
            let mut img = heic::load_heic(&bytes[0])?;
            // heic

            meta.image.width = img.0;
            meta.image.height = img.1;
            meta.resize();

            mem::swap(bytes, &mut img.2);

            Ok(meta)
        }

        ImgFormat::Aseprite => {
            let mut img = ase::load_ase(&bytes[0])?;

            meta.image = img.0;
            meta.resize();

            mem::swap(bytes, &mut img.1);

            Ok(meta)
        }

        ImgFormat::Gif => {
            let mut img = gif::load_gif(bytes[0].as_slice())?;

            meta.image = img.0;
            meta.resize();

            mem::swap(bytes, &mut img.1);

            Ok(meta)
        }

        _ => Err(crate::utils::err::MyErr::Null(())),
    }
}

#[inline(always)]
pub fn resize_page(
    page: &mut Page,
    img: &mut Vec<Vec<u8>>,
    meta: &MetaSize<u32>,
    filter: &FilterType,
) {
    match img.len() {
        1 => {
            let mut tmp = resize::resize_rgba8(mem::take(&mut img[0]), meta, filter).unwrap();

            page.data = vec![vec![]; 1];
            resize::srgb_u32(&mut page.data[0], &mem::take(&mut tmp));
        }

        _ => {
            let mut tmp: Vec<u8> = Vec::new();

            page.nums = img.len();
            page.data = vec![vec![]; img.len()];

            for idx in 0..img.len() {
                tmp = resize::resize_rgba8(mem::take(&mut img[idx]), meta, filter).unwrap();

                resize::srgb_u32(&mut page.data[idx], &mem::take(&mut tmp));
            }
        }
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
