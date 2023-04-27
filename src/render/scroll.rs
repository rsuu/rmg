use crate::{
    match_event, sleep, AsyncTask, Buffer, Canvas, Config, Data, ForAsyncTask, Img, KeyMap, Map,
    Page, PageList, TMetaSize, TransRgba,
};

#[derive(Debug)]
pub struct Scroll {
    pub buffer: Buffer,      //
    pub buffer_size: usize,  //
    pub buffer_limit: usize, //
    pub bit_len: usize,      //
    pub map: Map,            // user input

    pub cur: usize,    // =0
    pub rng: usize,    // =0
    pub head: usize,   // =0
    pub tail: usize,   // =1
    pub y_step: usize, // move_down AND move_up
    pub x_step: usize, // move_left AND move_right

    pub page_list: PageList,    //
    pub page_loading: Vec<u32>, //
    pub null_line_size: usize,

    pub window_position: (i32, i32), //
}

///////////////////////////////////////
impl Scroll {
    pub fn new(
        data: &Data,
        page_list: PageList,
        buffer_size: usize,
        config: &Config,
        null_line_size: usize,
    ) -> Self {
        let mem = {
            use sysinfo::SystemExt;

            let sys = sysinfo::System::new_all();

            sys.total_memory() as usize
        };

        Self {
            buffer: Buffer::new(),
            buffer_size,
            buffer_limit: buffer_size * config.base.limit as usize,

            cur: 0,

            head: 0,
            tail: 0,
            rng: 0,

            bit_len: 0,

            map: Map::Down,
            page_list,
            y_step: buffer_size / config.base.step as usize, // drop 1/step part of image once
            x_step: data.meta.window.width as usize / config.base.step as usize,
            window_position: (0, 0),

            null_line_size,
            page_loading: vec![TransRgba::rgba_as_argb_u32(&238, &238, &238, &128); buffer_size],
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

            sleep();
        }
    }

    ///
    #[inline(always)]
    fn mouse_input(&mut self, canvas: &mut Canvas, config: &Config) {
        // scroll
        if let Some((_, y)) = canvas.window.get_scroll_wheel() {
            //tracing::trace!("mouse_y == {}", y);

            match config.base.invert_mouse {
                true if y < 0.0 => self.move_up(),
                true if y > 0.0 => self.move_down(),

                false if y < 0.0 => self.move_down(),
                false if y > 0.0 => self.move_up(),

                _ => {}
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
        } else {
            return;
        }

        if self.map == Map::Down {
            self.load_next(arc_task);
            self.free_head(arc_task);
        } else if self.map == Map::Up {
            self.load_prev(arc_task);
            self.free_tail(arc_task);
        }

        self.buffer.free();

        for index in self.head..=self.tail {
            let len = self.page_list.get_ref(index).img.len();

            if self.page_list.get_ref(index).flush(&mut self.buffer.data) {
                self.page_list.get_mut(index).img.to_next_frame();
            } else if arc_task.try_set_as_todo(index) {
                for _ in 0..(len / self.null_line_size) {
                    self.buffer
                        .extend(&self.page_loading[0..self.null_line_size]);
                }
            } else {
            }
        }

        while self.buffer.len() < self.end() {
            self.buffer
                .extend(&self.page_loading[0..self.null_line_size]);
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

        if self.bit_len >= self.end() + self.y_step {
            self.rng += self.y_step;
        } else if self.bit_len >= self.buffer_size {
            self.rng = self.bit_len - self.buffer_size;
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
        self.map = Map::Left;

        // FIXME:
        if self.bit_len > self.end() + self.x_step && self.x_step <= data.meta.window.width as usize
        {
            self.rng += self.x_step;
        } else {
        }

        //tracing::debug!("start: {}", self.rng);
        //tracing::debug!("end: {}", self.end());
    }

    /// move right
    fn move_right(&mut self, data: &Data) {
        self.map = Map::Right;

        // FIXME:
        if self.rng >= self.x_step && self.x_step <= data.meta.window.width as usize {
            self.rng -= self.x_step;
        } else {
        }
    }

    #[inline(always)]
    fn end(&self) -> usize {
        self.rng + self.buffer_size
    }

    fn free_head(&mut self, arc_task: &AsyncTask) {
        let page_len = self.page_list.get_ref(self.head).img.len();

        if self.bit_len >= self.buffer_limit + page_len
            && self.tail > self.head
            && self.rng > page_len
            && arc_task.try_free(self.head, &mut self.page_list)
        {
            tracing::info!("free head");

            //self.page_list.free(self.head);
            self.head += 1;
            self.bit_len -= page_len;
            self.rng -= page_len;
        }
    }

    fn free_tail(&mut self, arc_task: &AsyncTask) {
        let page_len = self.page_list.get_ref(self.tail).img.len();

        if self.bit_len >= self.buffer_limit + page_len
            && self.tail > self.head
            && self.bit_len > page_len
            && arc_task.try_free(self.tail, &mut self.page_list)
        {
            tracing::info!("free tail");
            tracing::debug!(self.rng, self.bit_len, page_len);

            //self.page_list.free(self.tail);
            self.tail -= 1;
            self.bit_len -= page_len;
        }
    }

    fn load_next(&mut self, arc_task: &AsyncTask) {
        let page_len = self.page_list.get_ref(self.head).img.len();

        if self.bit_len < self.buffer_limit + page_len && self.tail + 1 < self.page_list.len() {
            self.tail += 1;
        }
    }

    fn load_prev(&mut self, arc_task: &AsyncTask) {
        let page_len = self.page_list.get_ref(self.tail).img.len();

        if self.bit_len < self.buffer_limit + page_len && self.head > 0 {
            self.head -= 1;
        }
    }

    pub fn load_from_mark(&mut self, cur: usize) {
        // TODO:
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
