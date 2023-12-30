use crate::{
    match_event, sleep, AsyncTask, Buffer, Canvas, Config, Data, ForAsyncTask, Img, KeyMap, Map,
    Page, PageList, TMetaSize, TransRgba,
};

#[derive(Debug)]
pub struct Scroll {
    pub buffer: Buffer,    //
    pub buffer_len: usize, //
    pub buffer_max: usize, //
    pub bit_len: usize,    //
    pub map: Map,          // keymap

    pub cur: usize,    // =0
    pub rng: usize,    // =0
    pub head: usize,   // =0
    pub tail: usize,   // =1
    pub y_step: usize, // move_down AND move_up
    pub x_step: usize, // move_left AND move_right

    pub page_list: PageList,    //
    pub page_loading: Vec<u32>, //
    pub empty_x_size: usize,    //

    pub window_position: (i32, i32), //
}

impl Scroll {
    pub fn new(
        data: &Data,
        page_list: PageList,
        buffer_len: usize,
        config: &Config,
        empty_x_size: usize,
    ) -> Self {
        let mem = {
            use sysinfo::System;

            let sys = sysinfo::System::new_all();

            sys.total_memory() as usize
        };

        Self {
            buffer: Buffer::new(),
            buffer_len,
            buffer_max: buffer_len * config.base.limit as usize,

            cur: 0,

            head: 0,
            tail: 0,
            rng: 0,

            bit_len: 0,

            map: Map::Down,
            page_list,
            // drop 1/step part of image once
            y_step: buffer_len / config.base.step as usize,
            x_step: data.meta.window.width as usize / config.base.step as usize,
            window_position: (0, 0),

            empty_x_size,

            // TODO: ?Anime
            page_loading: vec![TransRgba::rgba_as_argb_u32(&238, &238, &238, &128); buffer_len],
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

        while canvas.window.is_open() {
            self.update_len();

            match match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
                Map::Down => {
                    self.move_down();
                }

                Map::Up => {
                    self.move_up();
                }

                Map::Left => {
                    self.move_left(data);
                }

                Map::Right => {
                    self.move_right(data);
                }

                Map::Reset => {
                    todo!()
                }

                Map::FullScreen => {
                    todo!()
                }

                Map::Exit => {
                    // FIXME: Key::Escape
                    break;
                }

                _ => self.mouse_input(canvas, config),
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

                false if y > 0.0 => self.move_up(),
                false if y < 0.0 => self.move_down(),

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
                    tmp += self.page_list.get_ref(index).len();
                }
            }

            println!("mark: {}", self.cur);
        }
    }

    #[inline(always)]
    fn flush(&mut self, canvas: &mut Canvas, data: &Data, arc_task: &AsyncTask) {
        // false: do nothing
        if !arc_task.try_flush(&mut self.page_list) {
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
            let len = self.page_list.get_ref(index).len();

            // true: Anim.to_next_frame()
            if self.page_list.get_ref(index).flush(&mut self.buffer.data) {
                self.page_list.get_mut(index).img.to_next_frame();

            // TODO: remove this     // (stop now)
            //       add more RAM and CPU for rmg (SPACE, POWER)
            //       or stopping (FEEL BAD)
            //       VS
            //       gen_empty(w, h) // (scroll anywhere)
            //       higher I/O (TIME)
            // false: empty page
            } else if arc_task.try_set_as_todo(index) {
                for _ in 0..(len / self.empty_x_size) {
                    self.buffer.extend(&self.page_loading[0..self.empty_x_size]);
                }
            }
        }

        // update
        while self.buffer.len() < self.end() {
            self.buffer.extend(&self.page_loading[0..self.empty_x_size]);
        }
        self.buffer.data.truncate(self.end());

        canvas.flush(&self.buffer.data[self.rng..self.end()]);
    }

    #[inline(always)]
    fn update_len(&mut self) {
        self.bit_len = 0;

        for index in self.head..=self.tail {
            self.bit_len += self.page_list.get_ref(index).len()
        }
    }

    /// move down
    #[inline(always)]
    fn move_down(&mut self) {
        // TODO: rewrite
        self.map = Map::Down;

        if self.bit_len >= self.end() + self.y_step {
            self.rng += self.y_step;
        } else if self.bit_len >= self.buffer_len {
            self.rng = self.bit_len - self.buffer_len;
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

        // FIXME: ???
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
        self.rng + self.buffer_len
    }

    fn free_head(&mut self, arc_task: &AsyncTask) {
        let page_len = self.page_list.get_ref(self.head).len();

        if self.bit_len >= self.buffer_max + page_len
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
        let page_len = self.page_list.get_ref(self.tail).len();

        if self.bit_len >= self.buffer_max + page_len
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
        let page_len = self.page_list.get_ref(self.head).len();

        if self.bit_len < self.buffer_max + page_len && self.tail + 1 < self.page_list.len() {
            self.tail += 1;
        }
    }

    fn load_prev(&mut self, arc_task: &AsyncTask) {
        let page_len = self.page_list.get_ref(self.tail).len();

        if self.bit_len < self.buffer_max + page_len && self.head > 0 {
            self.head -= 1;
        }
    }

    pub fn seek_to(&mut self, idx: usize) {
        todo!()
    }
}
