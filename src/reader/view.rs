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

    pub fn pad(&mut self, page: Page) {
        self.data.extend_from_slice(page.data().unwrap());
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
            _Scroll => {}
            _Manga => {}
            _Comic => {}
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

#[derive(Debug, Clone)]
pub enum Img {
    Bit(ImgBit),

    //Svg()
    Gif(ImgGif), // TODO: image::code
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ImgBit {
    pub data: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct ImgGif {
    pub data: Vec<Vec<u32>>,
    pub nums: usize,
    pub pos: usize,
    pub fps: usize,
    pub timer: usize,
}

#[derive(Debug, Clone)]
pub struct ImgSvg {}

impl Page {
    pub fn new(name: String, number: usize, pos: usize) -> Self {
        Self {
            name,
            number,
            pos,
            resize: (0, 0),
            img: Img::Unknown,
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
        match self.img {
            Img::Gif(ref mut gif) => {
                gif.to_next_frame();
            }
            _ => {}
        }
    }
}

impl ImgBit {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Img {
    pub fn data(&self) -> Option<&[u32]> {
        match self {
            Img::Bit(img) => Some(img.data.as_slice()),
            Img::Gif(gif) => Some(gif.data[gif.pos].as_slice()),
            _ => {
                panic!()
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            Img::Bit(img) => img.data.clear(),
            Img::Gif(gif) => gif.data.clear(),
            _ => {}
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Img::Bit(img) => img.data.len(),
            Img::Gif(gif) => gif.data[gif.pos].len(),
            _ => 0,
        }
    }
}

impl ImgGif {
    pub fn new(nums: usize) -> Self {
        Self {
            data: Vec::new(),
            nums,
            pos: 0,
            fps: 60,
            timer: 0,
        }
    }

    pub fn to_next_frame(&mut self) {
        if self.pos + 1 <= self.nums {
            self.pos += 1;
        } else {
            self.pos = 0;
        }
    }
}
