use crate::img::size::Size;
use crate::FPS;
use log::debug;

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

    // for gif only
    pub idx: usize, // index of frame
    pub timer: usize,
    pub miss: usize,
    pub pts: Vec<u32>, // pts = delay + fps
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImgType {
    Bit = 0,  // jpg / heic ...
    Anim = 1, // gif / aseprite
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

    Command,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ViewMode {
    #[default]
    Scroll,

    Image, // image OR gif

    Manga, // Left to Right
    Comic, // Right to Left
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
impl From<&str> for ImgFormat {
    fn from(value: &str) -> Self {
        match value {
            "jpg" => Self::Jpg,
            "png" => Self::Png,
            "heic" | "heif" => Self::Heic,
            "avif" => Self::Avif,
            "aseprite" => Self::Aseprite,
            "gif" => Self::Gif,
            "svg" | "xml" => Self::Svg,
            _ => Self::Unknown,
        }
    }
}

impl Page {
    pub fn new(name: String, number: usize, pos: usize) -> Self {
        Self {
            name,
            number,
            pos,
            resize: Size::new(0, 0),
            is_ready: false,

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
        if self.ty == ImgType::Anim {
            self.pts[self.idx] as usize
        } else {
            0
        }
    }

    pub fn size(&self) -> usize {
        self.resize.width as usize * self.resize.height as usize
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.resize = Size::new(width, height);
    }

    #[inline(always)]
    pub fn data(&self) -> &[u32] {
        self.data[self.idx].as_slice()
    }

    pub fn free(&mut self) {
        self.data.clear();
        self.data.shrink_to(0);
    }

    #[inline]
    pub fn len(&self) -> usize {
        // WARN: for bit AND anim
        if self.is_ready {
            self.data[0].len()
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn to_next_frame(&mut self) {
        if self.ty == ImgType::Anim {
            if self.timer >= (self.pts() as isize - self.miss as isize).abs() as usize {
                self.miss = self.timer - self.pts();

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
        }

        //        let add = self.ty as usize;
        //
        //        if self.idx + add < self.data.len() {
        //            self.idx += add;
        //        } else {
        //            self.idx = 0;
        //        }
    }
}
