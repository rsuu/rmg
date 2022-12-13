use crate::{
    archive::{self, ArchiveType},
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
};
use fir::FilterType;
use log::{debug, info};
use std::{
    mem,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread::spawn,
};

#[derive(Debug, Clone)]
pub struct ExtData {
    pub page: Page,

    pub archive_type: ArchiveType,
    pub path: PathBuf,
    pub screen_size: Size<u32>,
    pub window_size: Size<u32>,
    pub filter: FilterType,
    pub pos: usize,
}

impl ExtData {
    pub fn new(
        archive_type: ArchiveType,
        path: PathBuf,
        screen_size: Size<u32>,
        window_size: Size<u32>,
        filter: FilterType,
    ) -> Self {
        Self {
            page: Page::null(),
            archive_type,
            path,
            screen_size,
            window_size,
            filter,
            pos: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    Nothing,

    NextReady,
    NextDone,

    PrevReady,
    PrevDone,

    LoadNext,
    LoadPrev,

    DoneNext,
    DonePrev,
}

#[derive(Debug)]
pub struct Render {
    pub buffer: Buffer,
    pub buffer_max: usize,
    pub mem_limit: usize,

    pub head: usize, // =0
    pub tail: usize, // =0

    pub rng: usize, //
    pub load_next: bool,
    pub load_prev: bool,

    pub page_list: Vec<Page>,
    pub page_end: usize,

    pub archive_path: PathBuf,
    pub archive_type: ArchiveType,

    pub x_step: usize, // move_down AND move_up
    pub y_step: usize, // move_left AND move_right

    pub mode: Map, // [UP, DOWN, QUIT, ...]

    pub window_size: Size<u32>,      //
    pub screen_size: Size<u32>,      // not need
    pub window_position: (i32, i32), //

    pub filter: FilterType, // resize image

    pub len: usize,     // free up Bit
    pub all_len: usize, // free up Anim

    pub page_load_list: Vec<usize>,

    pub view_mode: ViewMode,

    pub page_number: u32,
    pub page_loading: Vec<u32>,
}

impl Render {
    pub fn new(
        page_list: Vec<Page>,
        archive_type: ArchiveType,
        path: impl AsRef<Path>,
        buffer_max: usize,
        step: usize,
        screen_size: Size<u32>,
        window_size: Size<u32>,
        view_mode: ViewMode,
        filter: FilterType,
    ) -> Self {
        Self {
            buffer: Buffer::new(),
            buffer_max,
            mem_limit: buffer_max * 5,

            head: 0,
            tail: 0,
            rng: 0,
            load_next: false,
            load_prev: false,

            len: 0,
            all_len: 0,

            archive_path: path.as_ref().to_path_buf(),
            archive_type,

            mode: Map::Stop,
            page_end: page_list.len(),
            page_list,
            y_step: buffer_max / step, // drop 1/step part of image once
            x_step: window_size.width as usize / step,
            window_position: (0, 0),

            screen_size,
            window_size,

            page_load_list: Vec::new(),
            filter,
            view_mode,

            page_number: 0,
            page_loading: vec![0; buffer_max * 4],
        }
    }

    /// init
    pub fn init(&mut self) {
        let mut tmp = (0, 0);

        let mut len = 0;
        let mut has_anim_file = 0;

        match self.page_list.len() {
            0 => {
                panic!()
            }
            1 => {
                self.view_mode = ViewMode::Image;
                tmp = self.load_next();
            }
            _ => {
                tmp = self.load_next();
                len += tmp.0;
                has_anim_file += tmp.1;

                'l1: while len <= self.mem_limit && self.tail + 1 < self.page_end {
                    self.tail += 1;

                    tmp = self.load_next();
                    len += tmp.0;
                    has_anim_file += tmp.1;
                }
            }
        }

        // image.len() < buffer.len()
        if has_anim_file >= 1 {
            self.view_mode = ViewMode::Page;
        } else {
        }

        "*** INIT ***"._info();

        debug!("    len = {}", len);
    }

    // use for init
    pub fn load_next(&mut self) -> (usize, usize) {
        let tail = &mut self.page_list[self.tail];
        let pos = tail.pos;

        let (ty, mut buffer, format) =
            load_file(self.archive_type, self.archive_path.as_path(), pos).unwrap();
        let (meta, pts) =
            load_img(format, &mut buffer, self.screen_size, self.window_size).unwrap();

        tail.ty = ty;
        tail.resize = meta.fix;
        tail.pts = pts;

        resize_page(tail, &mut buffer, &meta, &self.filter);

        tail.is_ready = true;

        (tail.len(), (tail.ty == ImgType::Anim) as usize)
    }

    #[inline(always)]
    pub fn flush(&mut self, canvas: &mut Canvas, _arc_state: &Arc<RwLock<State>>) {
        self.buffer.clear();
        self.len = self.page_list_len().0;
        self.all_len = self.page_list_len().1;

        for idx in self.page_load_list.iter() {
            if self.page_list[*idx].is_ready {
                self.buffer.flush(self.page_list[*idx].data());
                self.page_list[*idx].to_next_frame();
            } else {
                // self.buffer
                //     .data
                //     .extend_from_slice(&self.page_loading[0..self.buffer_max]);
            }
        }

        canvas.flush(&self.buffer.data[self.rng..self.rng + self.buffer_max]);

        // debug!("self.page_number = {}", self.page_number);
    }

    #[inline]
    pub fn page_list_len(&mut self) -> (usize, usize) {
        let (mut len, mut all_lan) = (0, 0);

        self.page_load_list.clear();

        for (idx, page) in self.page_list.iter().enumerate() {
            if page.len() > 0 {
                len += page.len();
                all_lan += page.all_len();
                self.page_load_list.push(idx);
            } else {
            }
        }

        (len, all_lan)
    }

    /// move down
    #[inline(always)]
    pub fn move_down(
        &mut self,
        arc_state: &Arc<RwLock<State>>,
        arc_extdata: &Arc<RwLock<ExtData>>,
    ) {
        "move_down()"._info();
        debug!(
            "{},
{}",
            self.rng, self.len
        );

        self.mode = Map::Down;

        // scrolling
        if self.rng + self.y_step <= self.len - self.buffer_max {
            self.load_next = false;
            self.rng += self.y_step;
        } else if self.rng <= self.len - self.buffer_max {
            self.load_next = true;
            self.rng = self.len - self.buffer_max;
        } else {
        };

        let head_len = self.page_list[self.head].len();

        if let Ok(mut arc_state) = arc_state.try_write() {
            match *arc_state {
                State::Nothing | State::PrevDone => {
                    // try to load next page
                    if self.tail + 1 < self.page_end && self.len <= self.mem_limit + head_len {
                        if let Ok(mut arc_extdata) = arc_extdata.try_write() {
                            info!("load next");
                            info!("{:?}", *arc_state);

                            self.tail += 1;

                            let idx = self.tail;
                            self.page_list[idx].is_ready = false;
                            arc_extdata.pos = self.page_list[idx].pos;

                            mem::swap(&mut self.page_list[idx], &mut arc_extdata.page);

                            *arc_state = State::LoadNext;
                        } else {
                            // wait
                        }
                    } else {
                        // nothing
                    }
                }
                _ => {}
            }
        } else {
            // wait
        }

        if let Ok(mut arc_state) = arc_state.try_write() {
            match *arc_state {
                State::DoneNext => {
                    debug!("state == {:?}", *arc_state);

                    if let Ok(mut arc_extdata) = arc_extdata.try_write() {
                        // swap page and arc_temp_page again
                        mem::swap(&mut self.page_list[self.tail], &mut arc_extdata.page);
                        self.page_list[self.tail].is_ready = true;

                        *arc_state = State::Nothing;

                        "*** NEXT ***"._info();
                    } else {
                        // wait
                    }
                }

                State::Nothing => {
                    debug!("state == {:?}", *arc_state);

                    if self.head + 1 < self.tail
                        && self.len >= self.mem_limit / 2 + head_len
                        && self.rng >= self.buffer_max * 2 + head_len
                    {
                        self.rng -= self.page_list[self.head].len();
                        self.page_list[self.head].free();
                        self.head += 1;

                        "*** FREE ***"._info();
                    } else {
                        // nothing
                    }
                }
                _ => {}
            }
        } else {
            // wait
        }
    }

    /// move up
    #[inline(always)]
    pub fn move_up(&mut self, arc_state: &Arc<RwLock<State>>, arc_extdata: &Arc<RwLock<ExtData>>) {
        info!("move_up()");

        self.mode = Map::Up;

        if self.rng >= self.y_step {
            self.load_prev = false;
            self.rng -= self.y_step;
        } else if self.rng >= 0 {
            self.load_prev = true;
            self.rng -= self.rng;
        } else {
            unreachable!()
        };

        let tail_len = self.page_list[self.tail].len();

        // load image
        if let Ok(mut arc_state) = arc_state.try_write() {
            match *arc_state {
                State::Nothing | State::DoneNext => {
                    // try to load prev page
                    if self.head >= 1 && self.len <= self.mem_limit + tail_len {
                        if let Ok(mut arc_extdata) = arc_extdata.try_write() {
                            info!("load prev");

                            self.head -= 1;

                            let idx = self.head;
                            self.page_list[idx].is_ready = false;

                            arc_extdata.pos = self.page_list[idx].pos;
                            mem::swap(&mut self.page_list[idx], &mut arc_extdata.page);

                            *arc_state = State::LoadPrev;
                        } else {
                            // wait
                        }
                    } else {
                        // nothing
                    }
                }
                _ => {}
            }
        } else {
            // wait
        }

        // display image
        if let Ok(mut arc_state) = arc_state.try_write() {
            debug!("state == {:?}", *arc_state);

            match *arc_state {
                State::DonePrev => {
                    debug!("state == {:?}", *arc_state);

                    if let Ok(mut arc_extdata) = arc_extdata.try_write() {
                        // swap page and arc_temp_page again
                        mem::swap(&mut self.page_list[self.head], &mut arc_extdata.page);

                        self.page_list[self.head].is_ready = true;

                        // a bit different with move_down()
                        self.rng += self.page_list[self.head].len();

                        *arc_state = State::Nothing;

                        "*** PREV ***"._info();
                    } else {
                        // wait
                    }
                }

                State::Nothing => {
                    // free up memory
                    if self.tail > self.head + 1
                        && self.len >= self.mem_limit / 2 + tail_len
                        && self.len >= self.rng + self.buffer_max * 2 + tail_len
                    {
                        self.page_list[self.tail].is_ready = false;
                        self.page_list[self.tail].free();
                        self.tail -= 1;

                        "*** FREE ***"._info();
                    } else {
                    }
                }
                _ => {}
            }
        } else {
            // wait
        }
    }

    /// move left
    pub fn move_left(&mut self) {
        // ??? How it works
        if self.len > self.end() + self.x_step && self.x_step <= self.window_size.width as usize {
            self.rng += self.x_step;

            debug!("start: {}", self.rng);
            debug!("end: {}", self.end());
        } else {
        }

        self.mode = Map::Left;
    }

    /// move right
    pub fn move_right(&mut self) {
        if self.rng >= self.x_step && self.x_step <= self.window_size.width as usize {
            self.rng -= self.x_step;
        } else {
        }

        self.mode = Map::Right;
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.rng + self.buffer_max
    }
}

///
#[inline(always)]
pub fn load_file(
    archive_type: ArchiveType,
    archive_path: &Path,
    page_pos: usize,
) -> Res<(ImgType, Vec<Vec<u8>>, ImgFormat)> {
    debug!("archive_type == {:?}", archive_type);

    let bytes = match archive_type {
        ArchiveType::Tar => {
            "load tar"._dbg();

            archive::tar::load_file(archive_path, page_pos).unwrap()
        }

        ArchiveType::Zip => {
            "load zip"._dbg();

            archive::zip::load_file(archive_path, page_pos).unwrap()
        }

        ArchiveType::Dir => {
            "load dir"._dbg();

            archive::dir::load_dir(archive_path, page_pos).unwrap()
        }

        ArchiveType::File => {
            "load file"._dbg();

            archive::dir::load_file(archive_path).unwrap()
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
        ImgFormat::Jpg | ImgFormat::Png | ImgFormat::Heic | ImgFormat::Avif | ImgFormat::Svg => {
            Ok((ImgType::Bit, bytes, format))
        }

        ImgFormat::Aseprite | ImgFormat::Gif => Ok((ImgType::Anim, bytes, format)),

        ImgFormat::Unknown => Err(MyErr::Todo),
    }
}

#[inline(always)]
pub fn load_img(
    format: ImgFormat,
    bytes: &mut Vec<Vec<u8>>,
    screen_size: Size<u32>,
    window_size: Size<u32>,
) -> Res<(MetaSize<u32>, Vec<u32>)> {
    let mut meta = MetaSize::<u32>::new(
        screen_size.width,
        screen_size.height,
        window_size.width,
        window_size.height,
        0,
        0,
    );

    let mut pts = vec![];

    match format {
        ImgFormat::Jpg | ImgFormat::Png => {
            let img = image::load_from_memory(&bytes[0])?;
            meta.image.width = img.width();
            meta.image.height = img.height();
            meta.resize();

            // BUG:
            mem::swap(bytes, &mut vec![img.to_rgba8().to_vec()]);

            Ok((meta, pts))
        }

        ImgFormat::Heic | ImgFormat::Avif => {
            let mut img = heic::load_heic(&bytes[0])?;
            // heic

            meta.image.width = img.0;
            meta.image.height = img.1;
            meta.resize();

            mem::swap(bytes, &mut img.2);

            Ok((meta, pts))
        }

        ImgFormat::Aseprite => {
            let mut anim = ase::load_ase(&bytes[0])?;

            meta.image = anim.0;
            meta.resize();

            mem::swap(bytes, &mut anim.1);

            Ok((meta, anim.2))
        }

        ImgFormat::Gif => {
            let mut anim = gif::load_gif(bytes[0].as_slice())?;

            meta.image = anim.0;
            meta.resize();

            mem::swap(bytes, &mut anim.1);

            Ok((meta, anim.2))
        }

        ImgFormat::Svg => {
            let mut img = crate::img::svg::load_svg(bytes[0].as_slice())?;

            meta.image = img.0;
            meta.resize();

            mem::swap(bytes, &mut img.1);

            Ok((meta, pts))
        }

        _ => Err(MyErr::Todo),
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

            page.data = vec![vec![]; 1]; // bit
            resize::argb_u32(&mut page.data[0], &mem::take(&mut tmp));
        }

        _ => {
            let mut tmp: Vec<u8> = Vec::new();

            page.data = vec![vec![]; img.len()]; // anim

            for idx in 0..img.len() {
                tmp = resize::resize_rgba8(mem::take(&mut img[idx]), meta, filter).unwrap();

                resize::argb_u32(&mut page.data[idx], &mem::take(&mut tmp));
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
