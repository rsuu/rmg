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
        self.data.extend_from_slice(page.data().unwrap());
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
    pub number: usize,      // page number
    pub pos: usize,         // index of image in the archive file
    pub resize: (u32, u32), // (width, height)
    //meta:MetaData
    pub data: Vec<Vec<u32>>, // Bit: data[0] OR Anim: data[head..tail]
    pub ty: ImgType,

    // for gif only
    pub nums: usize,
    pub frame_pos: usize,
    pub fps: usize,
    pub timer: usize,
}
// --------------------------
#[derive(Debug, Clone, Copy)]
pub enum ImgType {
    Bit,  // jpg
    Anim, // gif
    Svg,  //
}
// --------------------------

impl Page {
    pub fn new(name: String, number: usize, pos: usize) -> Self {
        Self {
            name,
            number,
            pos,
            resize: (0, 0),

            data: vec![Vec::new()],
            ty: ImgType::Bit,
            nums: 0,
            frame_pos: 0,
            fps: 0,
            timer: 0,
        }
    }

    pub fn new_bit() -> Self {
        let mut res = Self::null();
        res.ty = ImgType::Bit;

        res
    }

    pub fn new_anim() -> Self {
        let mut res = Self::null();
        res.ty = ImgType::Anim;

        res
    }

    pub fn null() -> Self {
        Self {
            name: "".to_string(),
            number: 0,
            pos: 0,
            resize: (0, 0),

            data: vec![Vec::new()],
            ty: ImgType::Bit,
            nums: 0,
            frame_pos: 0,
            fps: 0,
            timer: 0,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.resize = (width, height);
    }

    pub fn loading(&mut self) -> Vec<u32> {
        vec![0; self.resize.0 as usize * self.resize.1 as usize]
    }

    pub fn data(&self) -> Option<&[u32]> {
        match self.ty {
            ImgType::Bit => Some(self.data[0].as_slice()),
            ImgType::Anim => Some(self.data[self.frame_pos].as_slice()),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
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
        match self.ty {
            ImgType::Anim => {
                if self.frame_pos + 1 <= self.nums {
                    self.frame_pos += 1;
                } else {
                    self.frame_pos = 0;
                }
            }
            _ => {}
        }
    }
}
