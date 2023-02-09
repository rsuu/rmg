use crate::{
    archive::utils::ArchiveType,
    img::resize,
    img::size::{MetaSize, Size, TMetaSize},
    utils::traits::ExtImageType,
    FPS,
};
use fir::FilterType;
use imagesize;
use std::{
    mem,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tracing::debug;

// ==============================================
pub type Frame = Vec<u32>;
pub type Frames = Vec<Frame>;
pub type AsyncTask = Arc<RwLock<Task>>;

// ==============================================

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

pub trait ForAsyncTask {
    fn new(list: Vec<TaskResize>) -> AsyncTask;

    fn try_set_as_todo(&self, index: usize) -> bool;
    fn try_start(&self, data: &Data) -> bool;
    fn try_check(&self, index: usize) -> bool;
    fn try_flush(&self, list: &mut PageList) -> bool;
    fn try_free(&self, index: usize) -> bool;
}

impl ForAsyncTask for AsyncTask {
    fn new(list: Vec<TaskResize>) -> AsyncTask {
        Arc::new(RwLock::new(Task::new(list)))
    }

    fn try_set_as_todo(&self, index: usize) -> bool {
        if self.read().unwrap().list[index].state == State::Empty {
            let Ok(mut inner) = self.try_write()else { return false;};

            if inner.get_ref(index).state == State::Empty {
                inner.list[index].state = State::Todo;
            } else {
                return false;
            }
        }

        false
    }

    fn try_start(&self, data: &Data) -> bool {
        let Ok(mut inner) =self.try_write()else {
            return false;
        };

        for (index, task) in inner.list.iter_mut().enumerate() {
            if task.state == State::Todo {
                task.page.load_file(data).unwrap();

                //inner.ram_usage += inner.get_ref(index).page.len();
                task.state = State::Done;

                return true;
            } else {
                tracing::debug!("{} : {:?}", index, task.state);
            }
        }

        false
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
            tracing::debug!(
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
                page.data = mem::take(&mut inner.list[index].page.data);
                page.is_ready = true;
                page.size = inner.list[index].page.size;
                page.resize = inner.list[index].page.resize;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Check {
    Head,
    Body,
    Tail,
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

#[derive(Debug, Clone)]
pub struct PageList {
    pub list: Vec<Page>,
    pub head: usize,
    pub tail: usize,
    pub cur: usize,

    //pub buffer_max:usize,
    pub idx_loading: Option<usize>,
}

impl PageList {
    pub fn new(list: Vec<Page>) -> Self {
        let mut res = Vec::with_capacity(list.len());

        res.extend(list.into_iter());

        for (idx, page) in res.iter_mut().enumerate() {
            page.number = idx;
        }

        tracing::debug!("list: {:?}", &res);

        Self {
            list: res,
            head: 0,
            cur: 1,
            tail: 2,

            idx_loading: None,
        }
    }

    pub fn get_ref(&self, idx: usize) -> &Page {
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

    pub fn set_loading(&mut self) {
        self.idx_loading = Some(self.cur);
    }

    pub fn done(&mut self) {
        self.idx_loading = None;
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
    pub data: Frames, // Bit: data[0] OR Anim: data[head..tail]
    pub name: String,
    pub number: usize, // page number
    pub ty: ImgType,
    pub fmt: ImgFormat,

    // Arc
    pub is_ready: bool, // reset after thread return ok()

    // use once
    pub archive_pos: usize, // index of image in the archive file
    pub size: Size<u32>,    //
    pub resize: Size<u32>,  // (fix_width, fix_height)

    pub offset_x: usize,
    pub frames: usize,

    // for gif only
    pub frame_idx: usize, // index of frame
    pub timer: usize,
    pub miss: usize,
    pub pts: Vec<u32>, // pts = delay + fps
    pub check: Check,
}

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32,
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
            check: Check::Body,
            offset_x: 0,

            data: vec![],
            pts: vec![],
            ty: ImgType::Bit,
            fmt: ImgFormat::Unknown,
            frame_idx: 0,
            timer: 0,
            miss: 0,
            frames: 0,
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
            check: Check::Body,
            offset_x: 0,

            data: vec![],
            pts: vec![],
            ty: ImgType::Bit,
            fmt: ImgFormat::Unknown,
            frame_idx: 0,
            timer: 0,
            miss: 0,
            frames: 0,
        }
    }

    #[inline]
    pub fn pts(&self) -> usize {
        self.pts[self.frame_idx] as usize
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
    pub fn flush(&self, buffer: &mut Vec<u32>, slice: &[u32]) -> bool {
        if self.is_ready {
            buffer.extend_from_slice(self.data[self.frame_idx].as_slice());
            true
        } else {
            buffer.extend_from_slice(&slice[0..self.resize.len()]);
            false
        }
    }

    pub fn page_len(&self) -> usize {
        self.data[self.frame_idx].len()
    }

    pub fn data_crop(&self, ww: usize) {
        // TODO:
        let _res = resize::crop_img2(
            self.data[self.frame_idx].as_slice(),
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
    pub fn len(&self) -> usize {
        if self.is_ready {
            if self.ty == ImgType::Anim {
                self.data[0].len() * self.data.len()
            } else {
                self.data[0].len()
            }
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn to_next_frame(&mut self) {
        if self.ty == ImgType::Anim {
            if self.timer >= (self.pts() as isize - self.miss as isize).unsigned_abs() {
                self.miss = self.timer.saturating_sub(self.pts());

                if self.frame_idx + 1 < self.data.len() {
                    self.frame_idx += 1;
                } else {
                    // reset
                    self.frame_idx = 0;
                    self.timer = 0;
                }
            } else {
                self.timer += FPS as usize;
            }

            debug!("self.timer = {}", self.timer);
            debug!("self.frame_idx = {}", self.frame_idx);
            debug!("self.miss = {}", self.miss);
            debug!("self.pts() = {}", self.pts());
            debug!(
                "self.data.len() = {}

                         ",
                self.data.len()
            );
        } else {
            // self.frame_idx = 0;
            // do nothing
        }
    }

    // ===========================================
    ///
    pub fn load_file(&mut self, data: &Data) -> anyhow::Result<()> {
        let mut buffer = vec![vec![]];

        self.decode_img(&mut buffer, data)?;
        self.resize_img(&mut buffer, data)?;

        tracing::trace!("{}", self.data.len());

        Ok(())
    }

    fn set_info(&mut self, buffer: &[u8], data: &Data) -> anyhow::Result<bool> {
        let fmt = imagesize::image_type(buffer)?.as_fmt();
        let ty = {
            match fmt {
                ImgFormat::Unknown => return Err(anyhow::anyhow!("")),
                ImgFormat::Aseprite | ImgFormat::Gif => ImgType::Anim,
                _ => ImgType::Bit,
            }
        };
        let size = {
            let tmp = imagesize::blob_size(buffer)?;
            Size::new(tmp.width as u32, tmp.height as u32)
        };

        self.fmt = fmt;
        self.ty = ty;
        self.size = size;

        Ok(true)
    }

    fn decode_img(&mut self, buffer: &mut Vec<Vec<u8>>, data: &Data) -> anyhow::Result<()> {
        use crate::{
            archive::{self, utils::ForExtract},
            img::*,
        };

        // load bytes from file
        buffer[0] = data.archive_type.get_file(&data.path, self.archive_pos)?;

        let file = buffer[0].as_slice();
        tracing::debug!("{}", file.len());

        self.set_info(file, data)?;

        match self.fmt {
            ImgFormat::Jpg | ImgFormat::Png => {
                let img = image::load_from_memory(file)?;

                tracing::info!("decode png");

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

                self.set_size(anim.0.width, anim.0.height);
                mem::swap(buffer, &mut anim.1);
                //self.pts = anim.2;
            }

            ImgFormat::Gif => {
                // TODO:
                let mut anim = gif::load_gif(file)?;

                self.set_size(anim.0.width, anim.0.height);
                mem::swap(buffer, &mut anim.1);
                self.pts = anim.2;
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

        Ok(())
    }

    fn resize_img(&mut self, img: &mut Vec<Vec<u8>>, data: &Data) -> anyhow::Result<()> {
        tracing::trace!("img: {}", img[0].len());

        // frames
        self.frames = img.len();
        self.data = vec![vec![]; self.frames];

        let size = self.get_resize();

        match (self.ty, img.len()) {
            (ImgType::Bit, 1) => {
                self.data = vec![vec![]; 1]; // bit

                resize::resize_rgba8(&mut img[0], &self.size, &self.resize, &data.filter)?;

                resize::argb_u32(&mut self.data[0], &mem::take(&mut img[0]));
            }

            (ImgType::Anim, _) => {
                let meta = data.meta;

                if size.width > meta.window.width {
                    todo!();

                    // FIXME: How to do?
                    // resize()
                    // for (frame_idx, frame) in img.iter_mut().enumerate() {
                    //     resize::argb_u32(&mut tmp_frame, frame.as_slice());
                    //
                    //     resize::crop_img(
                    //         &mut self.data[frame_idx],
                    //         &tmp_frame,
                    //         offset,
                    //         size.width as usize,
                    //         size.height as usize,
                    //         meta.window.width as usize,
                    //     );
                    //
                    //     self.data[frame_idx] = mem::take(&mut tmp_frame);
                    // }
                } else {
                    let offset = (meta.window.width as usize - size.width as usize) / 2;

                    tracing::debug!("{}, {}, {}", meta.window.width, size.width, offset);

                    let mut tmp: Vec<u32> = vec![];

                    for (frame_idx, frame) in img.iter_mut().enumerate() {
                        resize::argb_u32(&mut tmp, frame.as_slice());
                        resize::center_img(
                            &mut self.data[frame_idx],
                            &mem::take(&mut tmp),
                            meta.window.width as usize,
                            size.width as usize,
                            size.height as usize,
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    Empty,
    Todo,
    Done,
    Locked,
    NeedFree,
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
