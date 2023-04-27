pub mod display;
pub mod keymap;
pub mod window;

pub mod once;
pub mod scroll;
pub mod turn;

pub mod draw;

// ==============================================
use crate::{
    archive::*, img::*, mem, sleep_ms, thread, yield_now, Arc, FilterType, Path, PathBuf, RwLock,
    FPS,
};

// ==============================================
pub type Frame = Vec<u32>; // RGBA8
pub type Frames = Vec<Frame>;

pub type AsyncTask = Arc<RwLock<Task>>;

// ==============================================
/// Create a `loop thread`.
pub fn new_thread(arc_task: &AsyncTask, data: &Data) {
    let arc_task = arc_task.clone();
    let data = data.clone();
    let mut tmp_page = Page::default();

    let f = move || loop {
        if let Some(index) = arc_task.try_start(&data, &mut tmp_page) {
            tracing::info!(
                "\
Thread: {:?},
  page_index: {}",
                thread::current().id(),
                index
            );

            //
        } else {
            // ? yield_now();
            sleep_ms(100);
        }
    };

    thread::spawn(f);
}

// ==============================================

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

#[derive(Debug, Clone)]
pub struct Page {
    pub img: Img,
    pub archive_pos: usize, // index of image in the archive file
    pub name: String,       // path
    pub number: usize,      // page number
}

#[derive(Debug)]
pub struct Buffer {
    pub nums: usize,
    pub data: Frame,
    //scale:Scale,
}

// async
#[derive(Debug)]
pub struct Task {
    list: Vec<TaskResize>,
    ram_usage: usize,
}

// async
#[derive(Debug)]
pub struct TaskResize {
    pub state: State,
    pub page: Page,
}

// ==============================================
//  +--> Empty -> Todo -> ... -> NeedFree -->+
//  |                                        |
//  +--------------<---<---<-----------------+
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    Empty,
    Todo,
    Ing,
    Done,
    Locked,
    NeedFree,
}

#[derive(Debug, Clone)]
pub enum Img {
    Unknown,

    Bit {
        img: Frame,
        size: Size<u32>,
        resize: Size<u32>,
    },
    Anim {
        img: Frames,
        size: Size<u32>,
        resize: Size<u32>,

        frame_count: usize, //
        frame_index: usize, //
        pts: Vec<u32>,      //
        timer: u32,         //
        miss: u32,          //
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImgType {
    Bit,  // jpg / heic ...
    Anim, // gif / aseprite
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImgFormat {
    Unknown,

    // bit
    Heic,
    Avif,
    Jpg,
    Png,
    Svg,

    // anim
    Aseprite,
    Gif,
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

    Turn, // Manga: Left to Right
          // Comic: Right to Left
}

#[derive(Debug, Clone, Copy)]
enum CmdFn {
    Crop,
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
            img: Img::default(),
        }
    }
}

impl Page {
    pub fn new(name: String, archive_pos: usize) -> Self {
        Self {
            name,
            archive_pos,
            number: 0,
            img: Img::default(),
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

            ImgFormat::Unknown => todo!(),
        }

        meta.resize();

        self.img = match ImgType::from(&fmt) {
            ImgType::Bit => Img::new_bit(
                //
                Vec::with_capacity(buffer.len()),
                meta.image,
                meta.fix,
            ),

            ImgType::Anim => Img::new_anim(
                // frame_size * count
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
        let mut index: Option<usize> = None;
        if let Ok(ref mut inner) = self.try_write() {
            for (idx, task) in inner.list.iter_mut().enumerate() {
                if task.state == State::Todo {
                    mem::swap(&mut task.page, tmp);
                    task.state = State::Ing;
                    index = Some(idx);

                    // Just load only one page.
                    break;
                } else if task.state == State::NeedFree {
                    // NOTE: free up the memory.
                    task.page.img.free();
                    task.state = State::Empty;

                    continue;
                }
            }
        } else {
            return None;
        };

        let Some(index)=index else {return None;};

        // decode AND resize image
        tmp.load_file(data).expect("ERROR: load_file()");

        if let Ok(ref mut inner) = self.try_write() {
            //inner.ram_usage += tmp.img.len();

            let task = &mut inner.list[index];

            mem::swap(&mut task.page, tmp);
            task.state = State::Done;
            *tmp = Page::default();

            tracing::info!("DONE: {}", index);

            return Some(task.page.number);
        }

        None
    }

    fn try_flush(&self, list: &mut PageList) -> bool {
        let Ok(ref mut inner) = self.try_write() else {return false;};

        for (index, task_page) in list.list.iter_mut().enumerate() {
            match inner.get_ref(index).state {
                State::Done => {
                    inner.list[index].state = State::Locked;
                    // Now, page.img is null.
                    task_page.img = mem::take(&mut inner.list[index].page.img);
                }

                _ => {}
            }
        }

        true
    }

    fn try_free(&self, index: usize, list: &mut PageList) -> bool {
        let Ok(ref mut inner) = self.try_write() else {return false;};

        if inner.get_ref(index).state == State::Locked {
            inner.list[index].state = State::NeedFree;

            // move page.img to task.img
            // (task.img, page.img)
            mem::swap(&mut inner.list[index].page.img, &mut list.list[index].img);

            // NOTE: We need to free up the memory in try_start().

            true
        } else {
            false
        }
    }
}

impl Img {
    pub fn new_bit(img: Frame, size: Size<u32>, resize: Size<u32>) -> Self {
        Self::Bit { img, size, resize }
    }

    pub fn new_anim(img: Frames, pts: Vec<u32>, size: Size<u32>, resize: Size<u32>) -> Self {
        Self::Anim {
            frame_count: img.len(),
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
        tracing::debug!("size:   {:?}", self.ref_size());
        tracing::debug!("resize: {:?}", self.ref_resize());

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
                ref mut frame_count,
                ref size,
                ..
            } => {
                *img = vec![vec![]; bytes.len()];
                *frame_count = bytes.len();

                if size.width > data.meta.window.width {
                    *resize = Size::new(data.meta.window.width, size.height);

                    for (frame_index, frame) in bytes.iter_mut().enumerate() {
                        resize_rgba8(frame, &size, &resize, &data.filter).expect("");

                        argb_u32(&mut img[frame_index], &mem::take(frame));
                    }
                } else if size.width == data.meta.window.width {
                    for (frame_index, frame) in bytes.iter_mut().enumerate() {
                        argb_u32(&mut img[frame_index], frame.as_slice());
                    }
                } else if size.width < data.meta.window.width {
                    let bgw = data.meta.window.width as usize;
                    let fgw = size.width as usize;
                    let h = size.height as usize;

                    let mut fg: Frame = Vec::with_capacity(fgw * h);
                    for (frame_index, frame) in bytes.iter_mut().enumerate() {
                        argb_u32(&mut fg, frame.as_slice());
                        img[frame_index] = vec![0; bgw * h];
                        center_img(&mut img[frame_index], &fg, bgw, fgw, h);
                    }

                    *resize = Size::new(bgw as u32, h as u32);
                }
            }

            _ => unreachable!(),
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

    #[inline(always)]
    pub fn to_next_frame(&mut self) {
        match *self {
            Img::Anim {
                ref mut frame_index,
                ref mut timer,
                ref img,
                ref pts,
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

            _ => &[],
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
        Self::Unknown
    }
}
