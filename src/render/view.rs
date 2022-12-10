use crate::{archive::ArchiveType, img::resize, img::size::Size, FPS};
use fir::FilterType;
use log::debug;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

struct RgbaBuf {
    data: Vec<u32>,
    size: Size<u32>,
}

struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Check {
    Head,
    Body,
    Tail,
}

// use in the thread
#[derive(Debug, Clone)]
pub struct ArcTmpBuffer {
    pub page: Page,
    pub pos: usize,
}

impl ArcTmpBuffer {
    pub fn new() -> Self {
        Self {
            page: Page::null(),
            pos: 0,
        }
    }

    pub fn new_arc() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new()))
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    pub page: Page,
    pub pos: usize,
    pub archive_type: ArchiveType,
    pub path: PathBuf,
    pub screen_size: Size<u32>,
    pub window_size: Size<u32>,
    pub filter: FilterType,
}

impl Data {
    pub fn new(
        archive_type: ArchiveType,
        path: PathBuf,
        screen_size: Size<u32>,
        window_size: Size<u32>,
        filter: FilterType,
    ) -> Self {
        Self {
            page: Page::null(),
            pos: 0,
            archive_type,
            path,
            screen_size,
            window_size,
            filter,
        }
    }
}

#[derive(Debug)]
pub struct PageList {
    pub list: Vec<Page>,
    pub head: usize,
    pub tail: usize,
    pub cur: usize,
}

impl PageList {
    pub fn new(tmp_list: Vec<Page>) -> Self {
        let mut list = Vec::with_capacity(tmp_list.len() + 2);

        list.push(Page::null());
        list.extend(tmp_list.into_iter());
        list.push(Page::null());

        list.first_mut().unwrap().check = Check::Head;
        list.last_mut().unwrap().check = Check::Tail;

        Self {
            list,
            cur: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn get(&self, idx: usize) -> &Page {
        &self.list[idx]
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut Page {
        self.list.get_mut(idx).unwrap()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn swap(&mut self, l: usize, r: usize) {
        self.list.swap(l, r);
    }
}

#[derive(Debug)]
pub struct Buffer {
    pub nums: usize,
    pub data: Vec<u32>,
    //scale:Scale,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub data: Vec<Vec<u32>>, // Bit: data[0] OR Anim: data[head..tail]
    pub name: String,
    pub number: usize, // page number
    pub ty: ImgType,

    // Arc
    pub is_ready: bool, // reset after thread return ok()

    // use once
    pub pos: usize,        // index of image in the archive file
    pub resize: Size<u32>, // (fix_width, fix_height)

    pub offset_x: usize,

    // for gif only
    pub idx: usize, // index of frame
    pub timer: usize,
    pub miss: usize,
    pub pts: Vec<u32>, // pts = delay + fps
    pub check: Check,
}

#[derive(Debug, Clone)]
struct CropBlock {
    //  (x , y)
    //     +--------------+
    //     |              |
    //     |              |
    //     |              |
    //     |              |
    //     +--------------+
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImgType {
    Bit,  // jpg / heic ...
    Anim, // gif / aseprite
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImgFormat {
    // bit
    Heic,
    Avif,
    Jpg,
    Png,
    Svg,

    // anim
    Aseprite,
    Gif,

    Unknown,
}

#[derive(Debug, Default)]
pub enum ReaderMode {
    #[default]
    View,

    Crop,

    Command,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ViewMode {
    #[default]
    Scroll,

    Once, // image OR gif

    Turn, //
          // Manga: Left to Right
          // Comic: Right to Left
}

////////////////////////////////
impl Buffer {
    pub fn new() -> Self {
        Self {
            nums: 0,
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn flush(&mut self, bytes: &[u32]) {
        self.data.extend_from_slice(bytes);
    }
}

impl Page {
    pub fn new(name: String, pos: usize) -> Self {
        Self {
            name,
            number: 0,
            pos,
            resize: Size::new(0, 0),
            is_ready: false,
            check: Check::Body,
            offset_x: 0,

            data: vec![],
            pts: vec![],
            ty: ImgType::Bit,
            idx: 0,
            timer: 0,
            miss: 0,
        }
    }

    pub fn null() -> Self {
        Self {
            name: "".to_string(),
            number: 0,
            pos: 0,
            resize: Size::new(0, 0),
            is_ready: false,
            check: Check::Body,
            offset_x: 0,

            data: vec![],
            pts: vec![],
            ty: ImgType::Bit,
            idx: 0,
            timer: 0,
            miss: 0,
        }
    }

    #[inline]
    pub fn pts(&self) -> usize {
        self.pts[self.idx] as usize
    }

    pub fn size(&self) -> usize {
        self.resize.width as usize * self.resize.height as usize
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.resize = Size::new(width, height);
    }

    pub fn get_resize(&mut self) -> Size<u32> {
        self.resize
    }

    #[inline(always)]
    pub fn data(&self) -> &[u32] {
        self.data[self.idx].as_slice()
    }

    pub fn data_crop(&self, ww: usize) {
        // TODO:
        let _res = resize::crop_img2(
            self.data[self.idx].as_slice(),
            self.offset_x,
            self.resize.width as usize,
            self.resize.height as usize,
            ww, // window_size.width
        );
    }

    pub fn free(&mut self) {
        self.is_ready = false;
        self.data.clear();
        self.data.shrink_to(0);
    }

    #[inline]
    pub fn anim_len(&self) -> usize {
        if self.is_ready {
            self.data[0].len() * self.data.len()
        } else {
            0
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        if self.is_ready {
            self.data[0].len()
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn to_next_frame(&mut self) {
        if self.ty == ImgType::Anim {
            if self.timer >= (self.pts() as isize - self.miss as isize).unsigned_abs() {
                self.miss = self.timer.saturating_sub(self.pts());

                if self.idx + 1 < self.data.len() {
                    self.idx += 1;
                } else {
                    // reset
                    self.idx = 0;
                    self.timer = 0;
                }
            } else {
                self.timer += FPS as usize;
            }

            debug!("self.timer = {}", self.timer);
            debug!("self.idx = {}", self.idx);
            debug!("self.miss = {}", self.miss);
            debug!("self.pts() = {}", self.pts());
            debug!(
                "self.data.len() = {}

                         ",
                self.data.len()
            );
        } else {
            // self.idx = 0;
            // do nothing
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    Nothing,

    LoadNext,
    LoadPrev,

    DoneNext,
    DonePrev,
}

impl State {
    pub fn as_i8(&self) -> i8 {
        match *self {
            Self::Nothing => 0,
            Self::DonePrev => 1,
            Self::DoneNext => 2,
            Self::LoadPrev => -1,
            Self::LoadNext => -2,
            _ => {
                unreachable!()
            }
        }
    }

    pub fn from_i8(v: &i8) -> Self {
        match *v {
            0 => Self::Nothing,
            1 => Self::DonePrev,
            2 => Self::DoneNext,
            -1 => Self::LoadPrev,
            -2 => Self::LoadNext,
            _ => {
                unreachable!()
            }
        }
    }
}

impl From<&str> for ImgFormat {
    fn from(value: &str) -> Self {
        match value {
            "jpg" => Self::Jpg,
            "png" => Self::Png,
            "heic" | "heif" => Self::Heic,
            "avif" => Self::Avif,
            "ase" | "aseprite" => Self::Aseprite,
            "gif" => Self::Gif,
            "svg" | "xml" => Self::Svg,
            _ => Self::Unknown,
        }
    }
}

// use for mem::take()
impl Default for Page {
    fn default() -> Self {
        Page::null()
    }
}
