// comic:
//   [0, 1, 2, 3, 4, 5]
//
// manga:
//   .step_by(2).map(|g| g.swap( left, right ));
//   [0, 1, 2, 3, 4, 5]
//
//
// comic + double:
//   [0, 1, 2, 3, 4, 5]
//    ^^^^  ^^^^  ^^^^
//     g1    g2    g3
//
// manga + double:
//   .map(|g| g.swap( left, right ));
//   [1, 0, 3, 2, 5, 4]
//    ^^^^  ^^^^  ^^^^
//     g1    g2    g3
//
//

use crate::{
    match_event,
    render::{scroll::Scroll, *},
    Canvas, Config, KeyMap, Map, FPS,
};
use std::{
    sync::{Arc, RwLock},
    thread::sleep_ms,
};

#[derive(Debug)]
pub struct Turn {
    pub buffer: Buffer,
    pub buffer_len: usize,

    pub page_list: PageList,

    pub cur: usize, //
    pub map: Map,

    pub rng: usize,
    pub y_step: usize,

    pub is_double_page: bool,
    pub is_manga_mode: bool,

    pub page_max: usize,

    pub head: usize,
    pub tail: usize,
}

impl Turn {
    pub fn from_scroll(scroll: Scroll) -> Self {
        Self {
            buffer: Buffer::new(),
            buffer_len: scroll.buffer_len,
            page_list: scroll.page_list,
            cur: 1,
            map: Map::Stop,
            rng: 0,
            y_step: scroll.y_step,

            page_max: 3,
            is_double_page: false,
            is_manga_mode: false,

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
        //            //tracing::trace!("try_flush()");
        //
        //            self.buffer.free();
        //
        //            for index in self.head..=self.tail {
        //                if self.page_list.get_ref(index).flush(&mut self.buffer.data) {
        //                    self.page_list.get_mut(index).to_next_frame();
        //                } else {
        //                    let _ = arc_task.try_set_as_todo(index);
        //                    //tracing::debug!("todo: {}", index);
        //
        //                    self.buffer
        //                        .extend(&self.page_loading[0..self.page_list.get_ref(index).len()]);
        //                }
        //
        //                //tracing::trace!("{:?}, {:?}", self.map, self.page_list.get_ref(index).resize);
        //            }
        //
        //            if self.try_free_page(arc_task) {
        //                //tracing::info!("try_free()");
        //            }
        //        }
        //
        //        while self.buffer.len() < self.rng + self.buffer_len {
        //            self.buffer.extend(&self.page_loading);
        //        }
        //
        //        canvas.flush(&self.buffer.data[self.rng..self.rng + self.buffer_len]);
    }

    /// move down
    #[inline(always)]
    fn move_down(&mut self) {
        self.map = Map::Down;

        // buffer = &[rng..rng+buffer_len]
        if self.rng + self.y_step <= self.buffer_len {
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

struct VecPage {
    inner: Vec<Page>,
}

struct VecGroup {
    inner: Vec<Group>,
}

struct Group {
    l: Page,
    r: Page,
}

impl VecPage {
    // manga
    fn swap_page(&mut self) {
        swap_page(&mut self.inner);
    }
}

impl VecGroup {
    // manga + double
    fn swap_group(&mut self) {
        for Group { l, r } in self.inner.iter_mut() {
            std::mem::swap(l, r);
        }
    }
}

fn swap_page<T>(vec: &mut Vec<T>) {
    let len = vec.len();
    let n = len / 2;

    let mut i = 0;
    for _ in 0..n {
        vec.swap(i, i + 1);

        i += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_page() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![2, 1, 4, 3, 6, 5];

        swap_page(&mut vec);

        assert_eq!(vec, expected);
    }
}
