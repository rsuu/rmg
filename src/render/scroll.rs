use crate::{
    archive::{self, ArchiveType},
    config::rsconf::Config,
    img::{
        ase, gif, heic, resize,
        size::{MetaSize, Size, TMetaSize},
        svg,
    },
    render::{
        keymap::{self, KeyMap, Map},
        view::{ArcTmpBuffer, Buffer, Check, Data, ImgFormat, ImgType, Page, PageList, State},
        window::Canvas,
    },
    sleep,
    utils::{
        err::{MyErr, Res},
        file,
    },
    FPS,
};
use fir::FilterType;
use std::{
    mem,
    path::Path,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct Scroll {
    pub buffer: Buffer,
    pub buffer_max: usize,
    pub bit_len: usize, // free up Bit
    pub mem_limit: usize,
    pub map: Map,                    // [UP, DOWN, QUIT, ...]
    pub window_position: (i32, i32), //

    pub rng: usize,    //
    pub head: usize,   // =0
    pub tail: usize,   // =0
    pub x_step: usize, // move_down AND move_up
    pub y_step: usize, // move_left AND move_right

    pub page_list: PageList,
    pub page_load_list: Vec<usize>,
    pub page_loading: Vec<u32>,
    pub page_number: usize,
}

///////////////////////////////////////

impl Scroll {
    pub fn new(data: &Data, page_list: PageList, buffer_max: usize, step: usize) -> Self {
        Self {
            buffer: Buffer::new(),
            buffer_max,
            mem_limit: buffer_max * 6,

            head: 0,
            tail: 0,
            rng: 0,

            bit_len: 0,

            map: Map::Stop,
            page_list,
            y_step: buffer_max / step, // drop 1/step part of image once
            x_step: data.window_size.width as usize / step,
            window_position: (0, 0),

            page_load_list: Vec::new(),

            page_number: 0,
            page_loading: vec![0; buffer_max],
        }
    }

    pub fn start(
        config: &Config,
        buf: &mut Scroll,
        canvas: &mut Canvas,
        keymaps: &[KeyMap],
        data: &Data,
        arc_state: &Arc<RwLock<State>>,
        arc_buffer: &Arc<RwLock<ArcTmpBuffer>>,
    ) {
        // WARN: new thread
        thread_resize_image(data, arc_state, arc_buffer);

        let mut time_start = std::time::Instant::now();
        let mut ms = FPS;
        let mut now = std::time::Instant::now();
        let mut count = 0;

        'l1: while canvas.window.is_open() {
            buf.pages_data_len();

            match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
                Map::Down => {
                    buf.move_down();
                }

                Map::Up => {
                    buf.move_up();
                }

                Map::Reset => {
                    todo!()
                }

                Map::FullScreen => {
                    todo!()
                }

                Map::Left => {
                    buf.move_left(data);
                }

                Map::Right => {
                    buf.move_right(data);
                }

                Map::Exit => {
                    println!("EXIT");

                    // TODO: Key::Escape
                    break 'l1;
                }

                _ => {
                    buf.mouse_input(canvas, config);
                }
            }

            buf.flush(canvas);
            buf.try_load_page(data, arc_state, arc_buffer);

            now = std::time::Instant::now();
            count = (now - time_start).as_millis() as u64;
            time_start = now;
            ms = FPS.checked_sub(count / 6).unwrap_or(10);

            sleep(ms);
        }
    }

    pub fn load_page_number(&mut self, _page_number: usize) {}

    /// input from mouse
    #[inline(always)]
    pub fn mouse_input(&mut self, canvas: &mut Canvas, config: &Config) {
        // scroll
        if let Some((_, y)) = canvas.window.get_scroll_wheel() {
            tracing::debug!("mouse_y == {}", y);

            if config.base.invert_mouse {
                if y > 0.0 {
                    self.move_up();
                } else if y < 0.0 {
                    self.move_down();
                } else {
                }
            } else {
                if y > 0.0 {
                    self.move_down();
                } else if y < 0.0 {
                    self.move_up();
                } else {
                }
            }
        } else {
        }

        //  TODO:
        // left click
        if canvas.window.get_mouse_down(minifb::MouseButton::Left) {
            let (count, number) = &mut (0, 0);
            let (x, y) = canvas
                .window
                .get_mouse_pos(minifb::MouseMode::Clamp)
                .unwrap();

            // TODO: fix
            // page number + offset_y
            for idx in self.page_load_list.iter() {
                if *count as f32 >= self.rng as f32 + y {
                    *number = self.page_list.get(*idx).number;
                } else {
                    *count += self.page_list.get(*idx).data().len();
                }
            }

            let number = match *number {
                0 => self.page_list.list.len() - 1,
                _ => *number,
            };

            tracing::debug!("offset_y: {}", y);
            //dbg!(number, self.page_number);
        } else {
        }
    }

    pub fn flush(&mut self, canvas: &mut Canvas) {
        let (count, number) = &mut (0, 0);

        self.buffer.clear();

        for idx in self.page_load_list.iter() {
            // flush buffer
            self.buffer.flush(self.page_list.get(*idx).data());

            // TODO: fix
            // page number
            if *count >= self.rng {
                *number = self.page_list.get(*idx).number;
            } else {
                *count += self.page_list.get(*idx).data().len();
            }

            // anim
            self.page_list.get_mut(*idx).to_next_frame();
        }

        self.page_number = match *number {
            0 => self.page_list.list.len() - 1,
            _ => *number,
        };

        while self.buffer.data.len() < self.buffer_max {
            self.buffer.data.extend_from_slice(&self.page_loading);
        }

        canvas.flush(&self.buffer.data[self.rng..self.rng + self.buffer_max]);
    }

    pub fn pages_data_len(&mut self) {
        let (mut bit_len, mut anim_len) = (0, 0);

        self.page_load_list.clear();

        for (idx, page) in self.page_list.list.iter().enumerate() {
            if page.is_ready && page.len() > 0 {
                bit_len += page.len();
                anim_len += page.anim_len();
                self.page_load_list.push(idx);
            } else {
            }
        }

        self.bit_len = bit_len;
    }

    /// move down
    pub fn move_down(&mut self) {
        tracing::debug!("{}, {}", self.rng, self.bit_len);

        self.map = Map::Down;

        // buffer = &[rng..rng+buffer_max]
        if self.rng + self.buffer_max + self.y_step <= self.bit_len {
            self.rng += self.y_step;
        } else if self.rng + self.buffer_max <= self.bit_len {
            self.rng = self.bit_len - self.buffer_max;
        } else {
        }
    }

    /// move up
    pub fn move_up(&mut self) {
        self.map = Map::Up;

        if self.rng >= self.y_step {
            self.rng -= self.y_step;
        } else {
            self.rng = 0;
        };
    }

    /// move left
    pub fn move_left(&mut self, data: &Data) {
        // TODO:
        self.map = Map::Left;

        // ??? How it works
        if self.bit_len > self.end() + self.x_step && self.x_step <= data.window_size.width as usize
        {
            self.rng += self.x_step;
        } else {
        }

        tracing::debug!("start: {}", self.rng);
        tracing::debug!("end: {}", self.end());
    }

    /// move right
    pub fn move_right(&mut self, data: &Data) {
        self.map = Map::Right;

        if self.rng >= self.x_step && self.x_step <= data.window_size.width as usize {
            self.rng -= self.x_step;
        } else {
        }
    }

    pub fn end(&self) -> usize {
        self.rng + self.buffer_max
    }

    pub fn page_list_tail(&self) -> usize {
        self.page_list.len()
    }

    pub fn not_tail(&self) -> bool {
        self.page_list.get(self.tail + 1).check != Check::Tail
    }

    pub fn not_head(&self) -> bool {
        self.page_list.get(self.head - 1).check != Check::Head
    }

    pub fn try_load_page(
        &mut self,
        data: &Data,
        arc_state: &Arc<RwLock<State>>,
        arc_buffer: &Arc<RwLock<ArcTmpBuffer>>,
    ) {
        tracing::debug!("{},{}", self.head, self.tail);

        match self.map {
            Map::Down if (self.need_load_next() || self.need_free_head()) => {
                self.try_load_page_next(data, arc_state, arc_buffer);
            }

            Map::Up if (self.need_load_prev() || self.need_free_tail()) => {
                self.try_load_page_prev(data, arc_state, arc_buffer);
            }

            _ => {}
        }
    }

    fn need_load_next(&self) -> bool {
        self.not_tail()
            && (self.bit_len <= self.mem_limit || self.rng + self.buffer_max == self.bit_len)
    }

    fn need_load_prev(&self) -> bool {
        self.not_head() && (self.bit_len <= self.mem_limit || self.rng == 0)
    }

    fn need_free_head(&self) -> bool {
        let len = self.page_list.get(self.head).len();

        self.tail > self.head
            && self.rng >= len
            && self.bit_len >= self.mem_limit / 2 + len
            && self.bit_len >= self.rng
    }

    fn need_free_tail(&self) -> bool {
        let len = self.page_list.get(self.tail).len();

        self.tail > self.head
            && self.bit_len >= self.mem_limit / 2 + len
            && self.bit_len >= self.rng + self.buffer_max + len
    }

    pub fn try_load_page_next(
        &mut self,
        _data: &Data,
        arc_state: &Arc<RwLock<State>>,
        arc_buffer: &Arc<RwLock<ArcTmpBuffer>>,
    ) {
        let _head_len = self.page_list.get(self.head).len() * 2;

        let Ok(mut arc_state) = arc_state.try_write() else { return; };

        match *arc_state {
            // resize image
            State::Nothing | State::DonePrev if (self.need_load_next()) => {
                let Ok(mut arc_buffer) = arc_buffer.try_write() else {return;};

                tracing::info!("load next");

                self.tail += 1;
                let idx = self.tail;

                self.page_list.get_mut(idx).is_ready = false;

                arc_buffer.pos = self.page_list.get(idx).pos;
                mem::swap(self.page_list.get_mut(idx), &mut arc_buffer.page);

                *arc_state = State::LoadNext;
            }

            // load next
            State::DoneNext => {
                let Ok(mut arc_buffer) = arc_buffer.try_write() else {return;};

                // swap page and buffer again
                mem::swap(self.page_list.get_mut(self.tail), &mut arc_buffer.page);
                self.page_list.get_mut(self.tail).is_ready = true;

                *arc_state = State::Nothing;
            }

            State::Nothing if (self.need_free_head()) => {
                self.rng -= self.page_list.get(self.head).len();
                self.page_list.get_mut(self.head).free();
                self.head += 1;
            }

            _ => {}
        }
    }

    pub fn try_load_page_prev(
        &mut self,
        _data: &Data,
        arc_state: &Arc<RwLock<State>>,
        arc_buffer: &Arc<RwLock<ArcTmpBuffer>>,
    ) {
        let _tail_len = self.page_list.get(self.tail).len() * 2;

        let Ok(mut arc_state) = arc_state.try_write() else {return;};

        match *arc_state {
            State::Nothing | State::DoneNext if self.need_load_prev() => {
                let Ok(mut arc_buffer) = arc_buffer.try_write()  else {
                        return;
                    };

                tracing::info!("load prev");

                self.head -= 1;
                let idx = self.head;
                self.page_list.get_mut(idx).is_ready = false;

                arc_buffer.pos = self.page_list.get(idx).pos;
                mem::swap(self.page_list.get_mut(idx), &mut arc_buffer.page);

                *arc_state = State::LoadPrev;
            }

            State::DonePrev => {
                tracing::debug!("state == {:?}", *arc_state);

                let Ok(mut arc_buffer) = arc_buffer.try_write() else {
                    return;
                };

                // swap page and buffer again
                mem::swap(self.page_list.get_mut(self.head), &mut arc_buffer.page);

                self.page_list.get_mut(self.head).is_ready = true;
                self.rng += self.page_list.get(self.head).len();

                *arc_state = State::Nothing;
            }

            State::Nothing if self.need_free_tail() => {
                // free tail
                //self.rng -= self.page_list.get(self.tail).len();
                self.page_list.get_mut(self.tail).free();
                self.tail -= 1;
            }

            _ => {}
        }

        tracing::debug!("prev(): {}, {}", self.bit_len, self.rng);
    }
}

///////////////////////////////////////

///
pub fn load_file(
    archive_type: &ArchiveType,
    archive_path: &Path,
    page_pos: usize,
) -> Res<(ImgType, Vec<Vec<u8>>, ImgFormat)> {
    tracing::debug!("archive_type == {:?}", archive_type);

    let bytes = match *archive_type {
        ArchiveType::Tar => archive::tar::load_file(archive_path, page_pos).unwrap(),

        ArchiveType::Zip => archive::zip::load_file(archive_path, page_pos).unwrap(),

        ArchiveType::Dir => archive::dir::load_dir(archive_path, page_pos).unwrap(),

        ArchiveType::File => archive::dir::load_file(archive_path).unwrap(),
    };

    let bytes = vec![bytes];
    tracing::debug!("    len = {}", bytes.len());

    if bytes[0].is_empty() {
        return Err(MyErr::Todo);
    } else {
    }

    let mut format = ImgFormat::Unknown;

    if let Some(ty) = infer::get(&bytes[0]) {
        format = ImgFormat::from(ty.extension());
    } else if file::is_aseprite(&bytes[0]) {
        format = ImgFormat::Aseprite;
    } else {

        // FIXME:rmg -t svg xxx.svg
        // format = ImgFormat::Svg;
    }

    match format {
        ImgFormat::Jpg | ImgFormat::Png | ImgFormat::Heic | ImgFormat::Avif | ImgFormat::Svg => {
            Ok((ImgType::Bit, bytes, format))
        }

        ImgFormat::Aseprite | ImgFormat::Gif => Ok((ImgType::Anim, bytes, format)),

        ImgFormat::Unknown => Err(MyErr::Todo),
    }
}

pub fn load_img(
    format: &ImgFormat,
    bytes: &mut Vec<Vec<u8>>,
    screen_size: &Size<u32>,
    window_size: &Size<u32>,
) -> Res<(MetaSize<u32>, Vec<u32>)> {
    // bytes.len() == 1
    // bytes[0] = image_bytes

    let mut meta = MetaSize::<u32>::new(
        screen_size.width,
        screen_size.height,
        window_size.width,
        window_size.height,
        0,
        0,
    );

    let pts = vec![];

    match *format {
        ImgFormat::Jpg | ImgFormat::Png => {
            let img = image::load_from_memory(&bytes[0])?;
            meta.image.width = img.width();
            meta.image.height = img.height();
            meta.resize();

            mem::swap(bytes, &mut vec![img.to_rgba8().to_vec()]);

            Ok((meta, pts))
        }

        ImgFormat::Heic | ImgFormat::Avif => {
            let mut img = heic::load_heic(&bytes[0])?;
            // heic
            meta.image.width = img.0;
            meta.image.height = img.1;
            meta.resize();

            mem::swap(bytes, &mut img.2);

            Ok((meta, pts))
        }

        ImgFormat::Aseprite => {
            let mut anim = ase::load_ase(&bytes[0])?;

            meta.image = anim.0;
            meta.fix = anim.0;

            mem::swap(bytes, &mut anim.1);

            Ok((meta, anim.2))
        }

        ImgFormat::Gif => {
            // TODO:
            let mut anim = gif::load_gif(bytes[0].as_slice())?;

            meta.image = anim.0;
            meta.fix = anim.0;

            mem::swap(bytes, &mut anim.1);

            Ok((meta, anim.2))
        }

        ImgFormat::Svg => {
            let mut img = svg::load_svg(bytes[0].as_slice())?;

            meta.image = img.0;
            meta.resize();

            mem::swap(bytes, &mut img.1);

            Ok((meta, pts))
        }

        _ => Err(MyErr::Todo),
    }
}

pub fn resize_page(
    page: &mut Page,
    img: &mut Vec<Vec<u8>>,
    meta: &MetaSize<u32>,
    filter: &FilterType,
    window_size: &Size<u32>,
) {
    // frames
    page.data = vec![vec![]; img.len()];

    let size = page.get_resize();
    let mut tmp = Vec::with_capacity(size.len());

    match img.len() {
        1 => {
            let mut tmp = resize::resize_rgba8(mem::take(&mut img[0]), meta, filter).unwrap();

            page.data = vec![vec![]; 1]; // bit
            resize::argb_u32(&mut page.data[0], &mem::take(&mut tmp));
        }

        _ => {
            if size.width > window_size.width {
                // FIXME: How to do?
                // resize()

                todo!();

                for (frame_idx, frame) in img.iter_mut().enumerate() {
                    resize::argb_u32(&mut tmp, frame.as_slice());

                    // resize::crop_img(
                    //     &mut page.data[frame_idx],
                    //     &tmp,
                    //     offset++,
                    //     size.width as usize,
                    //     size.height as usize,
                    //     window_size.width as usize,
                    // );

                    page.data[frame_idx] = mem::take(&mut tmp);
                }
            } else {
                let offset = (window_size.width as usize - size.width as usize) / 2;

                tracing::debug!("{}, {}, {}", window_size.width, size.width, offset);

                for (frame_idx, frame) in img.iter_mut().enumerate() {
                    resize::argb_u32(&mut tmp, frame.as_slice());

                    resize::center_img(
                        &mut page.data[frame_idx],
                        &mem::take(&mut tmp),
                        window_size.width as usize,
                        size.width as usize,
                        size.height as usize,
                        offset,
                    );
                }
            }
        }
    }
}

pub fn thread_resize_image(
    data: &Data,
    arc_state: &Arc<RwLock<State>>,
    arc_buffer: &Arc<RwLock<ArcTmpBuffer>>,
) {
    let data = data.clone();
    let arc_state = arc_state.clone();
    let arc_buffer = arc_buffer.clone();

    // BUG:
    std::thread::spawn(move || {
        loop {
            if load_page(&data, &arc_state, &arc_buffer).is_some() {
            } else {
            }

            // limit CPU usage
            //std::thread::sleep(std::time::Duration::from_millis(1000));
            std::thread::yield_now();
        }
    });
}

pub fn load_page(
    data: &Data,
    arc_state: &Arc<RwLock<State>>,
    arc_buffer: &Arc<RwLock<ArcTmpBuffer>>,
) -> Option<(usize, ImgType)> {
    let Ok(mut arc_state) = arc_state.try_write() else {
        return None;
    };

    if (*arc_state == State::LoadNext) || (*arc_state == State::LoadPrev) {
        let Ok(mut arc_buffer) = arc_buffer.try_write() else {
            return None;
        };

        let (ty, mut buffer, format) =
            load_file(&data.archive_type, &data.path, arc_buffer.pos).unwrap();

        let (meta, pts) =
            load_img(&format, &mut buffer, &data.screen_size, &data.window_size).unwrap();

        arc_buffer.page.ty = ty;
        arc_buffer.page.resize = meta.fix;
        arc_buffer.page.pts = pts;

        resize_page(
            &mut arc_buffer.page,
            &mut buffer,
            &meta,
            &data.filter,
            &data.window_size,
        );

        *arc_state = State::from_i8(&arc_state.as_i8().abs());

        Some((arc_buffer.page.len(), arc_buffer.page.ty))
    } else {
        None
    }
}

// pub fn push_front<T>(vec: &mut Vec<T>, slice: &[T]) {
//     let amt = slice.len(); // [1, 2, 3]
//     let len = vec.len(); // [4, 5, 6]
//
//     vec.reserve(amt);
//
//     unsafe {
//         std::ptr::copy(vec.as_ptr(), vec.as_mut_ptr().offset((amt) as isize), len);
//         std::ptr::copy(slice.as_ptr(), vec.as_mut_ptr(), amt);
//
//         vec.set_len(len + amt);
//     }
// }
//
//
// pub fn free_head<T>(buffer: &mut Vec<T>, range: usize)
// where
//     T: Sized + Clone,
// {
//     buffer.drain(..range);
// }
//
//
// pub fn free_tail<T>(buffer: &mut Vec<T>, range: usize)
// where
//     T: Sized,
// {
//     buffer.truncate(buffer.len() - range);
// }
