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
    pub len: usize,             // width * heigth
    pub resize: (usize, usize), // (width, height)
    pub img: Img,
    //scale:Scale,
    //meta:MetaData
}

#[derive(Debug, Clone)]
pub enum Img {
    Bit(ImgBit),

    //Svg()
    Gif(ImgGif),
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
            len: 0,
            resize: (0, 0),
            img: Img::Unknown,
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
            _ => {
                todo!()
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Img::Bit(img) => img.data.len(),
            _ => {
                todo!()
            }
        }
    }
}
