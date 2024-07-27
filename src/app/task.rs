use crate::*;

use std::sync::{Arc, OnceLock, RwLock};

pub static THREAD_GET_ALL_FRAME_SIZE: OnceLock<Vec<Page>> = OnceLock::new();

pub struct Pool {
    inner: rayon::ThreadPool,
    list: Arc<RwLock<Vec<Page>>>,
}

impl Pool {
    pub fn new(elems: Vec<Page>) -> Self {
        Self {
            inner: rayon::ThreadPoolBuilder::new().build().unwrap(),
            list: Arc::new(RwLock::new(elems)),
        }
    }

    pub fn task_load(&self, page: &mut Page, data: Arc<DataType>) {
        let index = page.index;
        let list = self.list.clone();

        self.inner.spawn(move || {
            let Ok(list) = &mut list.try_write() else {
                return;
            };
            let task = &mut list[index];

            if task.frame.size.is_zero() {
                task.load(&data, true).unwrap();
                // dbg!(task.frame.size);
            }
        });

        let Ok(list) = self.list.try_read() else {
            return;
        };
        let task = &list[index];

        if !task.frame.size.is_zero() {
            page.frame.size = task.frame.size;
            page.state = State::Loading;
        }
    }

    pub fn task_resize(&self, page: &mut Page, data: Arc<DataType>, config: &Config) {
        let list = self.list.clone();

        let algo_image = config.page_img_resize_algo();
        let algo_anim = config.page_anim_resize_algo();

        let index = page.index;
        let dst_size = page.dst_size;

        self.inner.spawn(move || {
            let Ok(list) = &mut list.try_write() else {
                return;
            };
            let task = &mut list[index];

            if task.state == State::Empty && !dst_size.is_zero() {
                let algo = {
                    if task.frame.ty() == FrameTy::Img {
                        algo_image
                    } else {
                        algo_anim
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

            // TODO: Align
            // let ew = page.dst_size.width();
            // let padding_left = center_x(cw, ew);
            // page.frame.vertex = page.frame.vertex.translate(padding_left, 0.0);
        }
    }
}
