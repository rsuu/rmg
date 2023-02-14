use crate::{
    config::rsconf::Config,
    render::{
        keymap::{match_event, KeyMap, Map},
        scroll::Scroll,
        utils::*,
        window::Canvas,
    },
    FPS,
};
use std::{
    sync::{Arc, RwLock},
    thread::sleep_ms,
};

#[derive(Debug)]
pub struct Turn {
    pub buffer: Buffer,
    pub buffer_max: usize,

    pub page_list: PageList,

    pub cur: usize, //
    pub map: Map,

    pub rng: usize,
    pub y_step: usize,

    pub is_double_page: bool,
    pub is_manga: bool,

    pub page_max: usize,

    pub head: usize,
    pub tail: usize,
}

impl Turn {
    pub fn from_scroll(scroll: Scroll) -> Self {
        Self {
            buffer: Buffer::new(),
            buffer_max: scroll.buffer_max,
            page_list: scroll.page_list,
            cur: 1,
            map: Map::Stop,
            rng: 0,
            y_step: scroll.y_step,

            page_max: 3,
            is_double_page: false,
            is_manga: false,

            head: 1,
            tail: 2,
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
                    //self.move_left(data);
                }

                Map::Right => {
                    //self.move_right(data);
                }

                Map::Exit => {
                    println!("EXIT");

                    // FIXME: Key::Escape
                    break 'l1;
                }

                _ => {
                    //self.mouse_input(canvas, config);
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

    fn to_next_page(&mut self) {}
    fn to_prev_page(&mut self) {}

    #[inline(always)]
    fn flush(&mut self, canvas: &mut Canvas, data: &Data, arc_task: &AsyncTask) {
        //        if arc_task.try_flush(&mut self.page_list) {
        //            tracing::trace!("try_flush()");
        //
        //            self.buffer.free();
        //
        //            for index in self.head..=self.tail {
        //                if self.page_list.get_ref(index).flush(&mut self.buffer.data) {
        //                    self.page_list.get_mut(index).to_next_frame();
        //                } else {
        //                    let _ = arc_task.try_set_as_todo(index);
        //                    tracing::debug!("todo: {}", index);
        //
        //                    self.buffer
        //                        .extend(&self.page_loading[0..self.page_list.get_ref(index).len()]);
        //                }
        //
        //                tracing::trace!("{:?}, {:?}", self.map, self.page_list.get_ref(index).resize);
        //            }
        //
        //            if self.try_free_page(arc_task) {
        //                tracing::info!("try_free()");
        //            }
        //        }
        //
        //        while self.buffer.len() < self.rng + self.buffer_max {
        //            self.buffer.extend(&self.page_loading);
        //        }
        //
        //        canvas.flush(&self.buffer.data[self.rng..self.rng + self.buffer_max]);
    }

    /// move down
    #[inline(always)]
    fn move_down(&mut self) {
        self.map = Map::Down;

        // buffer = &[rng..rng+buffer_max]
        if self.rng + self.y_step <= self.buffer_max {
            self.rng += self.y_step;
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
}
