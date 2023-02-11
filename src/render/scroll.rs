use crate::{
    config::rsconf::Config,
    img::utils::{TMetaSize, TransRgba},
    render::{
        keymap::{match_event, KeyMap, Map},
        utils::{AsyncTask, Buffer, Data, ForAsyncTask, Page, PageList},
        window::Canvas,
    },
    FPS,
};
use std::thread::sleep_ms;

#[derive(Debug)]
pub struct Scroll {
    pub buffer: Buffer,    //
    pub buffer_max: usize, //
    pub bit_len: usize,    // free up Bit
    pub mem_limit: usize,  //
    pub map: Map,          // [UP, DOWN, QUIT, ...]
    pub window_position: (i32, i32),

    pub need_load_prev: bool, // =0
    pub need_load_next: bool, // =0
    //
    pub cur: usize, // =0
    //
    pub rng: usize,    //
    pub head: usize,   // =0
    pub tail: usize,   // =0
    pub x_step: usize, // move_down AND move_up
    pub y_step: usize, // move_left AND move_right

    pub page_list: PageList,        //
    pub page_load_list: Vec<usize>, //
    pub page_loading: Vec<u32>,     //
    pub page_number: usize,         //
}

///////////////////////////////////////
impl Scroll {
    pub fn new(data: &Data, page_list: PageList, buffer_max: usize, config: &Config) -> Self {
        Self {
            buffer: Buffer::new(),
            buffer_max,
            mem_limit: buffer_max * config.base.limit as usize,

            cur: 0,

            need_load_next: false,
            need_load_prev: false,
            head: 0,
            tail: 0,
            rng: 0,

            bit_len: 0,

            map: Map::Down,
            page_list,
            y_step: buffer_max / config.base.step as usize, // drop 1/step part of image once
            x_step: data.meta.window.width as usize / config.base.step as usize,
            window_position: (0, 0),

            page_load_list: Vec::new(),

            page_number: 0,
            page_loading: vec![TransRgba::rgba_as_argb_u32(&255, &238, &238, &238); buffer_max],
        }
    }

    pub fn start(
        &mut self,
        config: &Config,
        canvas: &mut Canvas,
        keymaps: &[KeyMap],
        data: &Data,
        arc_task: &AsyncTask,
    ) {
        let mut time_start = std::time::Instant::now();
        let mut now = std::time::Instant::now();
        let mut ms = 0_u32;
        let mut count = 0;

        arc_task.try_set_as_todo(0);
        arc_task.try_set_as_todo(1);

        'l1: while canvas.window.is_open() {
            self.pages_len();

            match match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
                Map::Down => {
                    self.move_down();
                }

                Map::Up => {
                    self.move_up();
                }

                Map::Reset => {
                    todo!()
                }

                Map::FullScreen => {
                    todo!()
                }

                Map::Left => {
                    self.move_left(data);
                }

                Map::Right => {
                    self.move_right(data);
                }

                Map::Exit => {
                    println!("EXIT");

                    // FIXME: Key::Escape
                    break 'l1;
                }

                _ => {
                    self.mouse_input(canvas, config);
                }
            }

            self.flush(canvas, data, arc_task);
            self.map = Map::Stop;

            now = std::time::Instant::now();
            count = (now - time_start).as_millis() as u32;
            time_start = now;
            ms = FPS.checked_sub(count / 6).unwrap_or(10);

            sleep_ms(ms);
        }
    }

    fn load_page_number(&mut self, _page_number: usize) {}

    ///
    #[inline(always)]
    fn mouse_input(&mut self, canvas: &mut Canvas, config: &Config) {
        // scroll
        if let Some((_, y)) = canvas.window.get_scroll_wheel() {
            log::trace!("mouse_y == {}", y);

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
        }

        //  TODO:
        // left click
        if canvas.window.get_mouse_down(minifb::MouseButton::Left) {
            let (count, number) = &mut (0, 0);
            let (_x, y) = canvas
                .window
                .get_mouse_pos(minifb::MouseMode::Clamp)
                .unwrap();

            // TODO: fix
            // page number + offset_y
            for index in self.page_load_list.iter() {
                if *count as f32 >= self.rng as f32 + y {
                    *number = self.page_list.get_ref(*index).number;
                } else {
                    *count += self.page_list.get_ref(*index).len();
                }
            }

            let _number = match *number {
                0 => self.page_list.list.len() - 1,
                _ => *number,
            };

            log::debug!("offset_y: {}", y);
            //dbg!(number, self.page_number);
        }
    }

    #[inline(always)]
    fn flush(&mut self, canvas: &mut Canvas, data: &Data, arc_task: &AsyncTask) {
        if arc_task.try_flush(&mut self.page_list) {
            log::trace!("try_flush()");

            self.buffer.free();

            for index in self.head..=self.cur {
                if self.page_list.get_ref(index).flush(&mut self.buffer.data) {
                    self.page_list.get_mut(index).to_next_frame();
                } else {
                    let _ = arc_task.try_set_as_todo(index);
                    log::debug!("todo: {}", index);

                    self.buffer
                        .extend(&self.page_loading[0..self.page_list.get_ref(index).len()]);
                }

                log::trace!("{:?}, {:?}", self.map, self.page_list.get_ref(index).resize);
            }

            if self.try_free_page(arc_task) {
                log::info!("try_free()");
            }
        }

        while self.buffer.len() < self.rng + self.buffer_max {
            self.buffer.extend(&self.page_loading);
        }

        canvas.flush(&self.buffer.data[self.rng..self.rng + self.buffer_max]);
    }

    #[inline(always)]
    fn pages_len(&mut self) {
        self.page_load_list.clear();
        self.bit_len = 0;

        for (index, page) in self.page_list.list.iter().enumerate() {
            if page.is_ready && page.len() > 0 {
                self.bit_len += page.len();
                self.page_load_list.push(index);
            }
        }
    }

    /// move down
    #[inline(always)]
    fn move_down(&mut self) {
        log::debug!("{}, {}", self.rng, self.bit_len);

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
    #[inline(always)]
    fn move_up(&mut self) {
        self.map = Map::Up;

        if self.rng >= self.y_step {
            self.rng -= self.y_step;
        } else {
            self.rng = 0;
        };
    }

    /// move left
    fn move_left(&mut self, data: &Data) {
        // TODO:
        self.map = Map::Left;

        // ??? How it works
        if self.bit_len > self.end() + self.x_step && self.x_step <= data.meta.window.width as usize
        {
            self.rng += self.x_step;
        } else {
        }

        log::debug!("start: {}", self.rng);
        log::debug!("end: {}", self.end());
    }

    /// move right
    fn move_right(&mut self, data: &Data) {
        self.map = Map::Right;

        if self.rng >= self.x_step && self.x_step <= data.meta.window.width as usize {
            self.rng -= self.x_step;
        } else {
        }
    }

    fn end(&self) -> usize {
        self.rng + self.buffer_max
    }

    fn page_list_tail(&self) -> usize {
        self.page_list.len()
    }

    fn try_free_page(&mut self, arc_task: &AsyncTask) -> bool {
        log::debug!(
            "
{:?}
bit_len:   {}
mem_limit: {}
rng: {}
",
            (self.head, self.cur),
            self.bit_len,
            self.mem_limit,
            self.rng,
        );

        // head
        //   min: 0
        //   max: tail - 1
        // tail
        //   min: 1
        //   max: len  - 1
        match self.map {
            Map::Down => {
                log::trace!("down");

                let page_len = self.page_list.get_ref(self.head).len();

                if self.bit_len < self.mem_limit && self.cur + 1 < self.page_list.len() {
                    self.cur += 1;
                }

                if self.bit_len >= self.mem_limit / 2 + page_len
                    && self.rng > page_len
                    && self.cur > self.head
                    && arc_task.try_free(self.head)
                {
                    self.rng -= page_len;
                    self.page_list.list[self.head].free();
                    self.head += 1;

                    log::info!("free head");
                }
            }

            Map::Up => {
                log::trace!("up");

                if self.bit_len < self.mem_limit && self.head > 0 {
                    self.head -= 1;
                }

                if self.bit_len >= self.mem_limit / 4
                    && self.cur > self.head
                    && arc_task.try_free(self.cur)
                {
                    self.page_list.list[self.cur].free();
                    self.cur -= 1;

                    log::info!("free tail");
                }
            }

            _ => {}
        }

        true
    }
}

///////////////////////////////////////
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
