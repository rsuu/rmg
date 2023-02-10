use crate::render::{
    keymap::{KeyMap, Map},
    scroll::Scroll,
    utils::*,
    window::Canvas,
};
use std::sync::{Arc, RwLock};

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
            map: Map::Right,
            rng: 0,
            y_step: scroll.y_step, // drop 1/step part of image once
            is_double_page: false,
            is_manga: false,
            page_max: 3,

            head: 1,
            tail: 2,
        }
    }

    pub fn start(&mut self) {}
}
