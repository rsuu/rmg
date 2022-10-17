use crate::img::size::Size;

// todo
const TEMP: &[u32; 900 * 900] = &[0; 900 * 900];

#[derive(Debug)]
pub struct Buffer {
    pub nums: usize,
    pub data: Vec<u32>,
    //scale:Scale,
}

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

    pub fn flush(&mut self, page: &Page) {
        let data = page.data();

        if data.is_empty() {
            let data = page.loading();
            self.data.extend_from_slice(&data);
        } else {
            self.data.extend_from_slice(data)
        };
    }
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

    Image,

    Manga, // Left to Right
    Comic, // Right to Left
}

// --------------------------
#[derive(Debug, Clone)]
pub struct Page {
    pub name: String,
    pub number: usize,     // page number
    pub pos: usize,        // index of image in the archive file
    pub resize: Size<u32>, // (fix_width, fix_height)
    //meta:MetaData
    pub data: Vec<Vec<u32>>, // Bit: data[0] OR Anim: data[head..tail]
    pub ty: ImgType,
    pub is_ready: bool,

    // for gif only
    pub nums: usize,
    pub frame_pos: usize,
    pub fps: usize,
    pub timer: usize,
}
// --------------------------
#[derive(Debug, Clone, Copy)]
pub enum ImgType {
    Bit = 0,  // jpg
    Anim = 1, // gif
}
// --------------------------
#[derive(Debug, Clone, Copy)]
pub enum ImgFormat {
    Heic,
    Avif,
    Jpg,
    Png,

    Aseprite,

    Unknown,
}
// --------------------------

impl From<&str> for ImgFormat {
    fn from(value: &str) -> Self {
        match value {
            "jpg" => Self::Jpg,
            "png" => Self::Png,
            "heic" | "heif" => Self::Heic,
            "avif" => Self::Avif,
            "aseprite" => Self::Aseprite,
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

            data: vec![Vec::new()],
            ty: ImgType::Bit,
            nums: 0,
            frame_pos: 0,
            fps: 0,
            timer: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.resize.width as usize * self.resize.height as usize
    }

    pub fn null() -> Self {
        Self {
            name: "".to_string(),
            number: 0,
            pos: 0,
            resize: Size::new(0, 0),
            is_ready: false,

            data: vec![Vec::new()],
            ty: ImgType::Bit,
            nums: 0,
            frame_pos: 0,
            fps: 0,
            timer: 0,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.resize = Size::new(width, height);
    }

    pub fn loading(&self) -> Vec<u32> {
        vec![0; self.resize.width as usize / 100 * self.resize.height as usize]
    }

    pub fn data(&self) -> &[u32] {
        if self.is_ready {
            self.data[self.frame_pos].as_slice()
        } else {
            TEMP
        }
    }

    pub fn free(&mut self) {
        match self.ty {
            ImgType::Bit => {
                self.data[0].clear();
                self.data[0].shrink_to(0);
            }
            ImgType::Anim => {
                self.data.clear();
                self.data.shrink_to(0);
            }
            _ => {}
        }
    }

    pub fn len(&self) -> usize {
        match self.ty {
            ImgType::Bit => self.data[0].len(),
            ImgType::Anim => self.data[self.frame_pos].len(),
            _ => 0,
        }
    }

    pub fn to_next_frame(&mut self) {
        // bit  = 0
        // anim = 1
        let add = self.ty as usize;

        if self.frame_pos + add < self.nums {
            self.frame_pos += add;
        } else {
            self.frame_pos = 0;
        }
    }
}
