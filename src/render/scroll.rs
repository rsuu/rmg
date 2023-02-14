use crate::{
    config::rsconf::Config,
    img::utils::{TMetaSize, TransRgba},
    render::{
        keymap::{match_event, KeyMap, Map},
        utils::{AsyncTask, Buffer, Data, ForAsyncTask, Img, Page, PageList},
        window::Canvas,
    },
    FPS,
};
use std::thread::sleep_ms;

#[derive(Debug)]
pub struct Scroll {
    pub buffer: Buffer,    //
    pub buffer_max: usize, //
    pub bit_len: usize,    //
    pub mem_limit: usize,  //
    pub map: Map,          // user input

    pub cur: usize,    // =0
    pub rng: usize,    // =0
    pub head: usize,   // =0
    pub tail: usize,   // =1
    pub y_step: usize, // move_down AND move_up
    pub x_step: usize, // move_left AND move_right

    pub page_list: PageList,    //
    pub page_loading: Vec<u32>, //

    pub window_position: (i32, i32), //
}

///////////////////////////////////////
impl Scroll {
    pub fn new(data: &Data, page_list: PageList, buffer_max: usize, config: &Config) -> Self {
        use sysinfo::SystemExt;

        let sys = sysinfo::System::new_all();
        let mem = sys.total_memory() as usize;

        let mut mem_limit = buffer_max * config.base.limit as usize;

        if mem_limit >= mem / 2 {
            println!(
                "WARN: mem_limit is {}, but total_memory is {}",
                mem_limit, mem
            );
        } else if mem_limit >= mem {
            println!(
                "ERROR: mem_limit is {}, but total_memory is {}",
                mem_limit, mem
            );

            mem_limit = mem / 2;
        }

        Self {
            buffer: Buffer::new(),
            buffer_max,
            mem_limit,

            cur: 0,

            head: 0,
            tail: 0,
            rng: 0,

            bit_len: 0,

            map: Map::Down,
            page_list,
            y_step: buffer_max / config.base.step as usize, // drop 1/step part of image once
            x_step: data.meta.window.width as usize / config.base.step as usize,
            window_position: (0, 0),

            page_loading: vec![
                TransRgba::rgba_as_argb_u32(&238, &238, &238, &128);
                2000 * 1000 * 10
            ],
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

            tracing::trace!("{} , {}", self.bit_len, self.mem_limit);

            sleep_ms(ms);
        }
    }

    ///
    #[inline(always)]
    fn mouse_input(&mut self, canvas: &mut Canvas, config: &Config) {
        // scroll
        if let Some((_, y)) = canvas.window.get_scroll_wheel() {
            tracing::trace!("mouse_y == {}", y);

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

        // left click
        if canvas.window.get_mouse_down(minifb::MouseButton::Left) {
            let mut tmp = 0;

            for index in self.head..=self.tail {
                if tmp <= self.rng {
                    self.cur = self.page_list.get_ref(index).number;
                    break;
                } else {
                    tmp += self.page_list.get_ref(index).img.len();
                }
            }

            println!("mark: {}", self.cur);
        }
    }

    #[inline(always)]
    fn flush(&mut self, canvas: &mut Canvas, data: &Data, arc_task: &AsyncTask) {
        if arc_task.try_flush(&mut self.page_list) {
            tracing::trace!("try_flush()");

            self.buffer.free();

            for index in self.head..=self.tail {
                if self.page_list.get_ref(index).flush(&mut self.buffer.data) {
                    self.page_list.get_mut(index).img.to_next_frame();
                } else if arc_task.try_set_as_todo(index) {
                    self.buffer
                        .extend(&self.page_loading[0..self.page_list.get_ref(index).img.len()]);
                } else {
                }
            }

            if self.map == Map::Down {
                self.free_head(arc_task);
            } else if self.map == Map::Up {
                self.free_tail(arc_task);
            }

            self.try_load_page(arc_task);
        }

        while self.buffer.len() < self.end() {
            self.buffer.extend(&self.page_loading);
        }

        self.buffer.data.truncate(self.end());

        canvas.flush(&self.buffer.data[self.rng..self.end()]);
    }

    #[inline(always)]
    fn pages_len(&mut self) {
        self.bit_len = 0;

        for index in self.head..=self.tail {
            self.bit_len += self.page_list.get_ref(index).img.len()
        }
    }

    /// move down
    #[inline(always)]
    fn move_down(&mut self) {
        self.map = Map::Down;

        if self.end() + self.y_step <= self.bit_len {
            self.rng += self.y_step;
        } else if self.end() <= self.bit_len {
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
        }
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

        tracing::debug!("start: {}", self.rng);
        tracing::debug!("end: {}", self.end());
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

    fn free_head(&mut self, arc_task: &AsyncTask) {
        let page_len = self.page_list.get_ref(self.head).img.len();

        if self.bit_len >= self.mem_limit / 2 + page_len
            && self.tail > self.head
            && self.rng > page_len
            && self.bit_len > page_len
            && arc_task.try_free(self.head)
        {
            self.rng -= page_len;
            self.page_list.free(self.head);
            self.head += 1;
            self.bit_len -= page_len;

            tracing::info!("free head");
        }
    }

    fn free_tail(&mut self, arc_task: &AsyncTask) {
        let page_len = self.page_list.get_ref(self.tail).img.len();

        if self.bit_len >= self.mem_limit / 4
            && self.tail > self.head
            && self.bit_len > page_len
            && arc_task.try_free(self.tail)
        {
            self.page_list.free(self.tail);
            self.tail -= 1;
            self.bit_len -= page_len;

            tracing::info!("free tail");
        }
    }

    fn try_load_page(&mut self, arc_task: &AsyncTask) {
        tracing::debug!(
            "
{:?}
bit_len:   {}
mem_limit: {}
rng: {}
",
            (self.head, self.tail),
            self.bit_len,
            self.mem_limit,
            self.rng,
        );

        // head
        //   min: 0
        //   max: tail - 1
        // tail
        //   min: head + 1
        //   max: len  - 1
        match self.map {
            Map::Down => {
                tracing::trace!("down");

                let page_len = self.page_list.get_ref(self.head).img.len();

                if self.bit_len < self.mem_limit + page_len && self.tail + 1 < self.page_list.len()
                {
                    self.tail += 1;
                }
            }

            Map::Up => {
                tracing::trace!("up");

                let page_len = self.page_list.get_ref(self.tail).img.len();

                if self.bit_len < self.mem_limit + page_len && self.head > 0 {
                    self.head -= 1;
                }
            }

            _ => {}
        }
    }

    pub fn load_from_mark(&mut self) {
        self.tail = self.cur;
        self.head = self.tail - 1;
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
// pub fn free_head<T>(buffer: &mut Vec<T>, len: usize)
// where
//     T: Sized + Clone,
// {
//     buffer.drain(0..len);
// }
//
//
// #[inline(always)]
// pub fn free_tail<T>(buffer: &mut Vec<T>, len: usize)
// where
//     T: Sized,
// {
//     buffer.truncate(buffer.len() - len);
// }
