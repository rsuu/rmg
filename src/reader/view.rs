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

#[derive(Debug, Default)]
pub enum ViewMode {
    #[default]
    Scroll,

    Manga, // Left to Right
    Comic, // Right to Left
}

impl ViewMode {
    pub fn reader(mode: &ViewMode) {
        match mode {
            Scroll => {}
            Manga => {}
            Comic => {}
        }

        todo!()
    }
}

// --------------------------
#[derive(Debug, Clone)]
pub struct Page {
    pub name: String,
    pub number: usize,          // page number
    pub pos: usize,             // index of image in the archive file
    pub resize: (usize, usize), // (width, height)
    pub img: Img,
    //meta:MetaData
}
// --------------------------
#[derive(Debug, Clone, Copy)]
pub enum ImgType {
    Bit,  // jpg
    Anim, // gif
    Svg,  //
}

#[derive(Debug, Clone)]
pub struct Img {
    pub data: Vec<Vec<u32>>, // Bit: data[0] OR Anim: data[head..tail]
    pub ty: ImgType,

    // for gif only
    pub nums: usize,
    pub pos: usize,
    pub fps: usize,
    pub timer: usize,
}
// --------------------------

impl Page {
    pub fn new(name: String, number: usize, pos: usize) -> Self {
        Self {
            name,
            number,
            pos,
            resize: (0, 0),
            img: Img::new_bit(),
        }
    }

    pub fn flush(&mut self, img: Img) -> usize {
        self.img = img;
        self.img.len()
    }

    pub fn clear(&mut self) {
        self.img.clear();
    }

    pub fn null() -> Self {
        Self::new("".to_string(), 0, 0)
    }

    pub fn len(&self) -> usize {
        self.img.len()
    }

    pub fn data(&self) -> Option<&[u32]> {
        self.img.data()
    }

    pub fn to_next_frame(&mut self) {
        self.img.to_next_frame();
    }
}

impl Img {
    pub fn new_bit() -> Self {
        Self {
            data: vec![Vec::new()],
            ty: ImgType::Bit,
            nums: 0,
            pos: 0,
            fps: 0,
            timer: 0,
        }
    }

    pub fn new_anim() -> Self {
        Self {
            data: Vec::new(),
            ty: ImgType::Anim,
            nums: 0,
            pos: 0,
            fps: 60,
            timer: 0,
        }
    }

    pub fn data(&self) -> Option<&[u32]> {
        match self.ty {
            ImgType::Bit => Some(self.data[0].as_slice()),
            ImgType::Anim => Some(self.data[self.pos].as_slice()),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
        match self.ty {
            ImgType::Bit => self.data[0].clear(),
            ImgType::Anim => self.data.clear(),
            _ => {}
        }
    }

    pub fn len(&self) -> usize {
        match self.ty {
            ImgType::Bit => self.data[0].len(),
            ImgType::Anim => self.data[self.pos].len(),
            _ => 0,
        }
    }

    pub fn to_next_frame(&mut self) {
        match self.ty {
            ImgType::Anim => {
                if self.pos + 1 <= self.nums {
                    self.pos += 1;
                } else {
                    self.pos = 0;
                }
            }
            _ => {}
        }
    }
}
