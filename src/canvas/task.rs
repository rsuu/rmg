use crate::*;

use pollster;
use std::sync::{Arc, OnceLock, RwLock};

pub static THREAD_GET_ALL_FRAME_SIZE: OnceLock<Pages> = OnceLock::new();

pub fn thread_get_all_frame_size(mut pages: Pages, data: Arc<DataType>) {
    // TODO: https://doc.rust-lang.org/std/async_iter/index.html
    for page in pages.iter_mut() {
        let index = page.index;

        page.set_size(&data, false).unwrap();
    }

    THREAD_GET_ALL_FRAME_SIZE.get_or_init(|| pages);
}

pub struct Pool {
    inner: rayon::ThreadPool,
    list: Arc<RwLock<Pages>>,
}

impl Pool {
    pub fn new(pages: Pages) -> Self {
        Self {
            inner: rayon::ThreadPoolBuilder::new().build().unwrap(),
            list: Arc::new(RwLock::new(pages)),
        }
    }

    pub fn task_set_size(&self, page: &mut Page, data: Arc<DataType>) {
        let index = page.index;
        let list = self.list.clone();

        self.inner.spawn(move || {
            let Ok(list) = &mut list.try_write() else {
                return;
            };
            let task = &mut list[index];

            if task.frame.size.is_zero() {
                task.set_size(&data, true).unwrap();
                // dbg!(task.frame.size);
            }
        });

        let Ok(list) = self.list.try_read() else {
            return;
        };
        let task = &list[index];

        if !task.frame.size.is_zero() {
            page.frame.size = task.frame.size;
            page.state = State::Waiting;
        }
    }

    pub fn task_resize(&self, page: &mut Page, data: Arc<DataType>, config: &Config) {
        let list = self.list.clone();

        let algo_image = config.page_image_resize_algo();
        let algo_anime = config.page_anime_resize_algo();

        let index = page.index;
        let dst_size = page.dst_size;

        self.inner.spawn(move || {
            let Ok(list) = &mut list.try_write() else {
                return;
            };
            let task = &mut list[index];

            if task.state == State::Empty && !dst_size.is_zero() {
                let algo = {
                    if task.frame.ty() == FrameTy::Image {
                        algo_image
                    } else {
                        algo_anime
                    }
                };

                task.dst_size = dst_size;
                task.resize(&data, algo).unwrap();
                task.state = State::Done;

                // dbg!("resize", task.index, task.frame.size);
            }
        });

        let Ok(list) = &mut self.list.try_write() else {
            return;
        };
        let task = &mut list[index];

        if task.state == State::Done {
            mem::swap(task, page);

            task.free();
        }
    }
}
