pub mod display;
pub mod keymap;
pub mod window;

pub mod once;
pub mod scroll;
pub mod turn;

pub mod draw;

// ==============================================
use crate::{archive::*, img::*, FPS};
use fir::FilterType;
use std::{
    mem,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread::{self, sleep_ms},
};

// ==============================================
pub type Frame = Vec<u32>; // RGBA8
pub type Frames = Vec<Frame>;
pub type AsyncTask = Arc<RwLock<Task>>;

// ==============================================
pub fn new_thread(arc_task: &AsyncTask, data: &Data) {
    let arc_task = arc_task.clone();
    let data = data.clone();

    let mut page = Page::default();

    let f = move || loop {
        if let Some(index) = arc_task.try_start(&data, &mut page) {
            tracing::info!("Thread: {:?}, task: {index}", thread::current().id(),);
        } else {
            sleep_ms(100);
        }
    };

    thread::spawn(f);
}

// ==============================================
// async
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    Empty,
    Todo,
    Doing,
    Done,
    Locked,
    NeedFree,
}

#[derive(Debug, Clone)]
pub enum Img {
    Init,
    Bit {
        img: Frame,
        size: Size<u32>,
        resize: Size<u32>,
    },
    Anim {
        img: Frames,
        size: Size<u32>,
        resize: Size<u32>,

        frames_count: usize, //
        frame_index: usize,  // index of frame
        pts: Vec<u32>,       //
        timer: u32,          //
        miss: u32,           //
    },
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

    Crop, // TODO:

    Command, // TODO:
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

// ==============================================
#[derive(Debug)]
pub struct Task {
    list: Vec<TaskResize>,
    ram_usage: usize,
}

#[derive(Debug)]
pub struct TaskResize {
    pub state: State,
    pub page: Page,
}

#[derive(Debug, Clone)]
pub struct BitData {
    pub data: Frame,
    pub size: Size<u32>,
    pub resize: Size<u32>,
}

#[derive(Debug, Clone)]
pub struct AnimData {
    pub data: Frames,
    pub size: Size<u32>,
    pub resize: Size<u32>,

    pub frames_count: usize,
    pub frame_index: usize, // index of frame
    pub pts: Vec<u32>,      // pts = delay + fps
    pub timer: u32,         //
    pub miss: u32,          //
}

// read only
#[derive(Debug, Clone)]
pub struct Data {
    pub archive_type: ArchiveType,
    pub path: PathBuf,
    pub meta: MetaSize<u32>,
    pub filter: FilterType,
}

#[derive(Debug, Clone)]
pub struct PageList {
    pub list: Vec<Page>,
    pub cur_dir: PathBuf,
}

#[derive(Debug)]
pub struct Buffer {
    pub nums: usize,
    pub data: Frame,
    //scale:Scale,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub img: Img,
    pub archive_pos: usize, // index of image in the archive file
    pub name: String,       // path
    pub number: usize,      // page number
}

// ==============================================
pub trait ForAsyncTask {
    fn new(list: Vec<TaskResize>) -> AsyncTask;
    fn try_set_as_todo(&self, index: usize) -> bool;
    fn try_start(&self, data: &Data, tmp: &mut Page) -> Option<usize>;
    fn try_flush(&self, list: &mut PageList) -> bool;
    fn try_free(&self, index: usize, list: &mut PageList) -> bool;
}

// ==============================================
impl TaskResize {
    pub fn new(page: Page) -> Self {
        Self {
            state: State::Empty,
            page,
        }
    }
}

impl Task {
    pub fn new(list: Vec<TaskResize>) -> Self {
        Self { list, ram_usage: 0 }
    }

    pub fn get_ref(&self, index: usize) -> &TaskResize {
        &self.list[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut TaskResize {
        &mut self.list[index]
    }
}

impl Data {
    pub fn new(
        archive_type: ArchiveType,
        path: PathBuf,
        meta: MetaSize<u32>,
        filter: FilterType,
    ) -> Self {
        Self {
            archive_type,
            meta,
            path,
            filter,
        }
    }
}

impl PageList {
    pub fn new(list: &mut Vec<Page>) -> Self {
        // sort by filename
        list.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

        for (index, page) in list.iter_mut().enumerate() {
            page.number = index;
        }

        Self {
            list: list.to_owned(),
            cur_dir: std::env::current_dir().expect(""),
        }
    }

    pub fn free(&mut self, index: usize) {
        self.list[index].img.free();
    }

    pub fn get_ref(&self, index: usize) -> &Page {
        &self.list[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Page {
        self.list.get_mut(index).unwrap()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn swap(&mut self, l: usize, r: usize) {
        self.list.swap(l, r);
    }
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

    pub fn free(&mut self) {
        self.data.clear();
    }

    pub fn extend(&mut self, slice: &[u32]) {
        self.data.extend_from_slice(slice);
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            number: 0,
            archive_pos: 0,
            img: Img::Init,
        }
    }
}

impl Page {
    pub fn new(name: String, archive_pos: usize) -> Self {
        Self {
            name,
            archive_pos,
            number: 0,
            img: Img::Init,
        }
    }

    #[inline(always)]
    pub fn flush(&self, buffer: &mut Frame) -> bool {
        let slice = self.img.ref_data();

        if slice.is_empty() {
            return false;
        }

        buffer.extend_from_slice(slice);

        true
    }

    // ===========================================
    ///
    pub fn load_file(&mut self, data: &Data) -> anyhow::Result<()> {
        use crate::{
            archive::{self, ForExtract},
            img::*,
        };

        let mut buffer: Vec<Vec<u8>> = vec![vec![]];

        buffer[0] = data.archive_type.get_file(&data.path, self.archive_pos)?;

        let file = buffer[0].as_slice();
        let fmt = get_img_format(file);

        let mut meta = data.meta;
        let mut pts = vec![];

        //tracing::debug!("{}", file.len());

        match fmt {
            ImgFormat::Jpg | ImgFormat::Png => {
                let img = image::load_from_memory(file)?;

                //tracing::info!("decode png");

                meta.image = Size::new(img.width(), img.height());

                mem::swap(&mut buffer, &mut vec![img.to_rgba8().to_vec()]);
            }

            ImgFormat::Heic | ImgFormat::Avif => {
                let mut img = heic::load_heic(file)?;

                meta.image = Size::new(img.0, img.1);

                mem::swap(&mut buffer, &mut img.2);
            }

            ImgFormat::Aseprite => {
                // TODO: pts
                let mut anim = ase::load_ase(file)?;

                pts = anim.2;
                meta.image = anim.0;

                mem::swap(&mut buffer, &mut anim.1);
            }

            ImgFormat::Gif => {
                // TODO:
                let mut anim = gif::load_gif(file)?;

                meta.image = anim.0;
                pts = anim.2;

                mem::swap(&mut buffer, &mut anim.1);
            }

            ImgFormat::Svg => {
                let mut img = svg::load_svg(file)?;

                meta.image = img.0;

                mem::swap(&mut buffer, &mut img.1);
            }

            // TODO:
            ImgFormat::Unknown => panic!(),
        }

        meta.resize();

        self.img = match ImgType::from(&fmt) {
            ImgType::Bit => Img::new_bit(Vec::with_capacity(buffer.len()), meta.image, meta.fix),
            ImgType::Anim => Img::new_anim(
                Vec::with_capacity(buffer.len() * buffer[0].len()),
                pts,
                meta.image,
                meta.fix,
            ),
        };

        self.img.resize(&mut buffer, &data);

        Ok(())
    }
}

// ==============================================
impl ForAsyncTask for AsyncTask {
    fn new(list: Vec<TaskResize>) -> AsyncTask {
        Arc::new(RwLock::new(Task::new(list)))
    }

    fn try_set_as_todo(&self, index: usize) -> bool {
        let Ok(ref mut inner) = self.try_write() else {return false;};

        if inner.get_ref(index).state == State::Empty {
            inner.list[index].state = State::Todo;

            true
        } else {
            false
        }
    }

    fn try_start(&self, data: &Data, tmp: &mut Page) -> Option<usize> {
        let mut idx: Option<usize> = None;

        if let Ok(ref mut inner) = self.try_write() {
            for (index, task) in inner.list.iter_mut().enumerate() {
                if task.state == State::Todo {
                    mem::swap(&mut task.page, tmp);
                    idx = Some(index);
                    task.state = State::Doing;

                    break;
                }
            }
        } else {
            return None;
        };

        let Some(index)=idx else {return None;};

        tmp.load_file(data).expect("ERROR: load_file()");

        if let Ok(ref mut inner) = self.try_write() {
            //inner.ram_usage += tmp.img.len();

            let task = &mut inner.list[index];

            mem::swap(&mut task.page, tmp);
            task.state = State::Done;
            *tmp = Page::default();

            return Some(task.page.number);
        }

        None
    }

    fn try_flush(&self, list: &mut PageList) -> bool {
        let Ok(ref mut inner) = self.try_write() else {return false;};

        for (index, page) in list.list.iter_mut().enumerate() {
            match inner.get_ref(index).state {
                State::Done => {
                    inner.list[index].state = State::Locked;

                    // NOTE: free up the memory
                    page.img = mem::take(&mut inner.list[index].page.img);
                }

                State::NeedFree => {
                    // NOTE: free up the memory
                    inner.list[index].page.img.free();
                    inner.list[index].state = State::Empty
                }
                _ => {}
            }
        }

        return true;
    }

    fn try_free(&self, index: usize, list: &mut PageList) -> bool {
        let Ok(ref mut inner) = self.try_write() else {return false;};

        if inner.get_ref(index).state == State::Locked {
            inner.list[index].state = State::NeedFree;
            mem::swap(&mut inner.list[index].page.img, &mut list.list[index].img);

            true
        } else {
            false
        }
    }
}

// ==============================================
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

impl From<&ImgFormat> for ImgType {
    fn from(value: &ImgFormat) -> Self {
        match value {
            ImgFormat::Jpg => Self::Bit,
            ImgFormat::Png => Self::Bit,
            ImgFormat::Heic => Self::Bit,
            ImgFormat::Avif => Self::Bit,
            ImgFormat::Svg => Self::Bit,

            ImgFormat::Aseprite => Self::Anim,
            ImgFormat::Gif => Self::Anim,

            ImgFormat::Unknown => panic!(),
        }
    }
}

impl Default for Img {
    fn default() -> Self {
        Self::Init
    }
}

impl Img {
    pub fn new_bit(img: Frame, size: Size<u32>, resize: Size<u32>) -> Self {
        Self::Bit { img, size, resize }
    }

    pub fn new_anim(img: Frames, pts: Vec<u32>, size: Size<u32>, resize: Size<u32>) -> Self {
        Self::Anim {
            frames_count: img.len(),
            frame_index: 0,
            timer: 0,
            miss: 0,
            resize,
            img,
            size,
            pts,
        }
    }

    pub fn len(&self) -> usize {
        match *self {
            Img::Bit { ref resize, .. } => resize.len(),
            Img::Anim { ref resize, .. } => resize.len(),
            _ => 0,
        }
    }

    pub fn resize(&mut self, bytes: &mut Vec<Vec<u8>>, data: &Data) {
        tracing::debug!("{}", &bytes[0].len());
        tracing::debug!("{:?}", self.ref_size());
        tracing::debug!("{:?}", self.ref_resize());

        match *self {
            Img::Bit {
                ref mut img,
                ref resize,
                ref size,
                ..
            } => {
                resize_rgba8(&mut bytes[0], size, resize, &data.filter).expect("");
                argb_u32(img, &mut bytes[0]);
            }
            Img::Anim {
                ref mut img,
                ref mut resize,
                ref mut frames_count,
                ref size,
                ..
            } => {
                // anim
                *img = vec![vec![]; bytes.len()];
                *frames_count = bytes.len();

                if size.width > data.meta.window.width {
                    *resize = Size::new(data.meta.window.width, size.height);

                    // WARN: unsafe
                    for (frame_index, frame) in bytes.iter_mut().enumerate() {
                        resize_rgba8(frame, &size, &resize, &data.filter).expect("");

                        argb_u32(&mut img[frame_index], &mem::take(frame));
                    }
                } else if size.width <= data.meta.window.width {
                    let offset = ((data.meta.window.width - size.width) / 2) as usize;

                    //tracing::debug!(
                    //                "
                    //window:   {}
                    //anim:     {}
                    //offset:   {}
                    //",
                    //                data.meta.window.width,
                    //                size.width,
                    //                offset
                    //            );

                    let bg_size = &data.meta.window;
                    let fg_size = size;

                    let mut fg_buffer: Frame = Vec::with_capacity(fg_size.len());

                    for (frame_index, frame) in bytes.iter_mut().enumerate() {
                        argb_u32(&mut fg_buffer, frame.as_slice());
                        center_img(&mut img[frame_index], &fg_buffer, bg_size, fg_size, offset);
                    }

                    *resize = *bg_size;
                }
            }
            _ => {}
        }
    }

    pub fn free(&mut self) {
        match *self {
            Img::Bit { ref mut img, .. } => {
                img.clear();
                img.shrink_to(0);
            }
            Img::Anim { ref mut img, .. } => {
                img.clear();
                img.shrink_to(0);
            }
            _ => {}
        }
    }

    pub fn ref_size(&self) -> &Size<u32> {
        match *self {
            Img::Bit { ref size, .. } => size,
            Img::Anim { ref size, .. } => size,
            _ => &Size {
                width: 0,
                height: 0,
            },
        }
    }

    pub fn ref_resize(&self) -> &Size<u32> {
        match *self {
            Img::Bit { ref resize, .. } => resize,
            Img::Anim { ref resize, .. } => resize,
            _ => &Size {
                width: 0,
                height: 0,
            },
        }
    }

    pub fn to_next_frame(&mut self) {
        match *self {
            Img::Anim {
                ref img,
                ref pts,
                ref mut timer,
                ref mut frame_index,
                ..
            } => {
                let pts = pts[*frame_index];

                if *timer >= pts {
                    *timer = timer.checked_sub(pts).unwrap_or(0);

                    if *frame_index + 1 < img.len() {
                        *frame_index += 1;
                    } else {
                        // reset
                        *frame_index = 0;
                    }
                } else {
                    *timer += FPS;
                }
            }
            _ => {}
        }
    }

    pub fn ref_data(&self) -> &[u32] {
        match *self {
            Img::Bit { ref img, .. } => img,
            Img::Anim {
                ref img,
                ref frame_index,
                ..
            } => {
                if img.is_empty() {
                    &[]
                } else {
                    img[*frame_index].as_slice()
                }
            }
            Img::Init => &[],
        }
    }
}
