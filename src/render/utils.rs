use crate::{archive::utils::*, img::utils::*, FPS};
use fir::FilterType;
use std::{
    mem,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, RwLock,
    },
    thread::{self, sleep_ms},
};

// ==============================================
pub type Frame = Vec<u32>;
pub type Frames = Vec<Frame>;
pub type AsyncTask = Arc<RwLock<Task>>;

// ==============================================
// async
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    Empty,
    Todo,
    Done,
    Locked,
    NeedFree,
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

// read only
#[derive(Debug, Clone)]
pub struct Data {
    pub page: Page,
    pub pos: usize,
    pub archive_type: ArchiveType,
    pub path: PathBuf,
    pub meta: MetaSize<u32>,
    pub filter: FilterType,
}

#[derive(Debug, Clone)]
pub struct PageList {
    pub list: Vec<Page>,
    pub head: usize,
    pub tail: usize,
    pub cur: usize,
}

#[derive(Debug)]
pub struct Buffer {
    pub nums: usize,
    pub data: Vec<u32>,
    //scale:Scale,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub is_ready: bool,      // reset after flush()
    pub data: Frames,        // Bit: data[0] OR Anim: data[head..tail]
    pub frames_count: usize, //
    pub archive_pos: usize,  // index of image in the archive file
    pub name: String,        // path
    pub number: usize,       // page number
    pub ty: ImgType,         //
    pub fmt: ImgFormat,      //
    pub size: Size<u32>,     //
    pub resize: Size<u32>,   // (fix_width, fix_height)
    pub offset_x: usize,     //

    // for gif only
    pub pts: Vec<u32>,      // pts = delay + fps
    pub frame_index: usize, // index of frame
    pub timer: usize,       //
    pub miss: usize,        //
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy)]
struct CropBlock {
    // (l.x , l.y)
    //      +--------------+
    //      |              |
    //      |              |
    //      |              |
    //      |              |
    //      |              |
    //      |              |
    //      |              |
    //      |              |
    //      +--------------+
    //                (r.x , r.y)
    l: Point,
    r: Point,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl CropBlock {
    pub fn new(lx: u32, ly: u32, offset_x: u32, offset_y: u32) -> Self {
        Self {
            l: Point::new(lx, ly),

            r: Point::new(lx + offset_x + offset_y, ly + offset_x + offset_y),
        }
    }
}

// ==============================================
pub trait ForAsyncTask {
    fn new(list: Vec<TaskResize>) -> AsyncTask;

    fn try_set_as_todo(&self, index: usize) -> bool;
    fn try_start(&self, data: &Data) -> Option<usize>;
    fn try_check(&self, index: usize) -> bool;
    fn try_flush(&self, list: &mut PageList) -> bool;
    fn try_free(&self, index: usize) -> bool;
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
            page: Page::null(),
            pos: 0,
            archive_type,
            meta,
            path,
            filter,
        }
    }
}

impl PageList {
    pub fn new(list: Vec<Page>) -> Self {
        let mut res = Vec::with_capacity(list.len());

        res.extend(list.into_iter());

        for (index, page) in res.iter_mut().enumerate() {
            page.number = index;
        }

        log::debug!("list: {:?}", &res);

        Self {
            list: res,
            head: 0,
            cur: 1,
            tail: 2,
        }
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
        self.data.truncate(0);
    }

    pub fn extend(&mut self, slice: &[u32]) {
        self.data.extend_from_slice(slice);
    }
}

impl Page {
    pub fn new(name: String, archive_pos: usize) -> Self {
        Self {
            name,
            number: 0,
            archive_pos,
            resize: Size::new(0, 0),
            size: Size::new(0, 0),
            is_ready: false,
            offset_x: 0,

            data: vec![],
            pts: vec![],
            ty: ImgType::Bit,
            fmt: ImgFormat::Unknown,
            frame_index: 0,
            timer: 0,
            miss: 0,
            frames_count: 0,
        }
    }

    pub fn null() -> Self {
        Self {
            name: "".to_string(),
            number: 0,
            archive_pos: 0,
            resize: Size::new(0, 0),
            size: Size::new(0, 0),
            is_ready: false,
            offset_x: 0,

            data: vec![],
            pts: vec![],
            ty: ImgType::Bit,
            fmt: ImgFormat::Unknown,
            frame_index: 0,
            timer: 0,
            miss: 0,
            frames_count: 0,
        }
    }

    #[inline]
    pub fn pts(&self) -> usize {
        self.pts[self.frame_index] as usize
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size = Size::new(width, height);
    }

    pub fn set_resize(&mut self, width: u32, height: u32) {
        self.resize = Size::new(width, height);
    }

    pub fn get_resize(&self) -> Size<u32> {
        self.resize
    }

    #[inline(always)]
    pub fn flush(&self, buffer: &mut Vec<u32>) -> bool {
        if self.is_ready {
            buffer.extend_from_slice(self.data[self.frame_index].as_slice());
            true
        } else {
            //buffer.extend_from_slice(&slice[0..self.resize.len()]);
            false
        }
    }

    pub fn page_len(&self) -> usize {
        self.data[self.frame_index].len()
    }

    pub fn data_crop(&self, ww: usize) {
        // TODO:
        let _res = crop_img2(
            self.data[self.frame_index].as_slice(),
            self.offset_x,
            self.resize.width as usize,
            self.resize.height as usize,
            ww, // window_size.width
        );
    }

    pub fn free(&mut self) {
        self.is_ready = false;
        self.frame_index = 0;
        self.timer = 0;
        self.miss = 0;

        self.data.clear();
        self.data.shrink_to(0);
    }

    #[inline]
    pub fn len(&self) -> usize {
        if self.is_ready {
            if self.ty == ImgType::Anim {
                // FIXME: freeup memory
                //self.data[0].len() * self.data.len()
                self.data[0].len()
            } else {
                self.data[0].len()
            }
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn to_next_frame(&mut self) {
        if self.is_ready && self.ty == ImgType::Anim {
            if self.timer >= (self.pts() as isize - self.miss as isize).unsigned_abs() {
                self.miss = self.timer.saturating_sub(self.pts());

                if self.frame_index + 1 < self.data.len() {
                    self.frame_index += 1;
                } else {
                    // reset
                    self.frame_index = 0;
                    self.timer = 0;
                }
            } else {
                self.timer += FPS as usize;
            }

            log::debug!("self.timer = {}", self.timer);
            log::debug!("self.frame_index = {}", self.frame_index);
            log::debug!("self.miss = {}", self.miss);
            log::debug!("self.pts() = {}", self.pts());
            log::debug!(
                "self.data.len() = {}

                         ",
                self.data.len()
            );
        } else {
            // self.frame_index = 0;
            // do nothing
        }
    }

    // ===========================================
    ///
    pub fn load_file(&mut self, data: &Data) -> anyhow::Result<()> {
        let mut buffer = vec![vec![]];

        self.decode_img(&mut buffer, data)?;
        self.resize_img(&mut buffer, data)?;

        log::trace!("{}", self.data.len());

        Ok(())
    }

    fn decode_img(&mut self, buffer: &mut Vec<Vec<u8>>, data: &Data) -> anyhow::Result<()> {
        use crate::{
            archive::{self, utils::ForExtract},
            img::*,
        };

        // load bytes from file
        buffer[0] = data.archive_type.get_file(&data.path, self.archive_pos)?;

        let file = buffer[0].as_slice();
        let fmt = get_img_format(file);

        self.ty = ImgType::from(&fmt);
        self.fmt = fmt;

        log::debug!("{}", file.len());

        match self.fmt {
            ImgFormat::Jpg | ImgFormat::Png => {
                let img = image::load_from_memory(file)?;

                log::info!("decode png");

                self.set_size(img.width(), img.height());

                mem::swap(buffer, &mut vec![img.to_rgba8().to_vec()]);
            }

            ImgFormat::Heic | ImgFormat::Avif => {
                let mut img = heic::load_heic(file)?;

                self.set_size(img.0, img.1);

                mem::swap(buffer, &mut img.2);
            }

            ImgFormat::Aseprite => {
                // TODO: pts
                let mut anim = ase::load_ase(file)?;

                buffer.clear();
                buffer.truncate(0);

                self.pts = anim.2;
                self.set_size(anim.0.width, anim.0.height);

                mem::swap(buffer, &mut anim.1);
                //self.pts = anim.2;
            }

            ImgFormat::Gif => {
                // TODO:
                let mut anim = gif::load_gif(file)?;

                self.set_size(anim.0.width, anim.0.height);
                self.pts = anim.2;

                mem::swap(buffer, &mut anim.1);
            }

            ImgFormat::Svg => {
                let mut img = svg::load_svg(file)?;

                self.set_size(img.0.width, img.0.height);

                mem::swap(buffer, &mut img.1);
            }

            // TODO:
            ImgFormat::Unknown => panic!(),
        }

        self.resize = {
            let mut meta = data.meta;
            meta.image = self.size;
            meta.resize();

            meta.fix
        };
        self.is_ready = true;

        Ok(())
    }

    fn resize_img(&mut self, img: &mut Vec<Vec<u8>>, data: &Data) -> anyhow::Result<()> {
        log::trace!("file_len: {}", img[0].len());

        self.frames_count = img.len();

        match (self.ty, img.len()) {
            (ImgType::Bit, 1) => {
                // bit
                self.data = vec![vec![]; 1];

                resize_rgba8(&mut img[0], &self.size, &self.resize, &data.filter)?;
                argb_u32(&mut self.data[0], &mem::take(&mut img[0]));
            }

            (ImgType::Anim, _) => {
                // anim
                self.data = vec![vec![]; self.frames_count];

                if self.size.width > data.meta.window.width {
                    todo!();

                // struct Img {
                //     data: Anim(frames_count),
                //     size: Size<u32>,
                //     offset: usize,
                // }
                // FIXME: ? resize() vs crop()
                // for (frame_index, frame) in img.iter_mut().enumerate() {
                //     argb_u32(&mut tmp_frame, frame.as_slice());
                //
                //     crop_img(
                //         &mut self.data[frame_index],
                //         &tmp_frame,
                //         offset,
                //         size.width as usize,
                //         size.height as usize,
                //         meta.window.width as usize,
                //     );
                //
                //     self.data[frame_index] = mem::take(&mut tmp_frame);
                // }
                } else if self.size.width <= data.meta.window.width {
                    let offset = ((data.meta.window.width - self.size.width) / 2) as usize;

                    log::debug!(
                        "
window:   {}
anim:     {}
offset:   {}
",
                        data.meta.window.width,
                        self.size.width,
                        offset
                    );

                    let mut fg: Vec<u32> = vec![];

                    for (frame_index, frame) in img.iter_mut().enumerate() {
                        argb_u32(&mut fg, frame.as_slice());
                        center_img(
                            &mut self.data[frame_index],
                            &mem::take(&mut fg),
                            &data.meta.window,
                            &self.size,
                            offset,
                        );
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }
}

// ==============================================
impl ForAsyncTask for AsyncTask {
    fn new(list: Vec<TaskResize>) -> AsyncTask {
        Arc::new(RwLock::new(Task::new(list)))
    }

    fn try_set_as_todo(&self, index: usize) -> bool {
        let Ok(mut inner) = self.try_write() else { return false; };

        if inner.get_ref(index).state == State::Empty {
            inner.list[index].state = State::Todo;

            true
        } else {
            false
        }
    }

    fn try_start(&self, data: &Data) -> Option<usize> {
        let Ok(mut inner) =self.try_write() else { return None; };

        for (index, task) in inner.list.iter_mut().enumerate() {
            if task.state == State::Todo {
                task.page.load_file(data).expect("ERROR: load_file()");
                //inner.ram_usage += inner.get_ref(index).page.len();
                task.state = State::Done;

                return Some(index);
            } else {
                log::debug!("{} : {:?}", index, task.state);
            }
        }

        None
    }

    fn try_check(&self, index: usize) -> bool {
        let Ok( inner) =self.try_read()else {
            return false;
        };

        if inner.get_ref(index).state == State::Locked {
            true
        } else {
            false
        }
    }

    fn try_flush(&self, list: &mut PageList) -> bool {
        let Ok(mut inner) = self.try_write() else {
            return false;
        };

        for (index, page) in list.list.iter_mut().enumerate() {
            log::debug!(
                "
index: {}
len: {}
state: {:?}
",
                index,
                inner.get_ref(index).page.len(),
                inner.get_ref(index).state
            );

            if inner.get_ref(index).state == State::Done {
                // base
                page.data = mem::take(&mut inner.list[index].page.data);
                page.is_ready = inner.list[index].page.is_ready;
                page.size = inner.list[index].page.size;
                page.resize = inner.list[index].page.resize;

                // gif
                page.fmt = inner.list[index].page.fmt;
                page.ty = inner.list[index].page.ty;
                page.pts = mem::take(&mut inner.list[index].page.pts);

                inner.get_mut(index).state = State::Locked;
            }
        }

        true
    }

    fn try_free(&self, index: usize) -> bool {
        let Ok(mut inner)=self.try_write()else {
            return true;
        };

        if inner.get_ref(index).state == State::Locked {
            inner.get_mut(index).page.free();
            inner.get_mut(index).state = State::Empty;

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

// use for mem::take()
impl Default for Page {
    fn default() -> Self {
        Page::null()
    }
}
