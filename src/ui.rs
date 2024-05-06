pub mod elem;
pub mod style;

use crate::*;

pub trait Element {
    fn draw(&self, canvas: &mut Canvas);
}

pub struct World<'canvas> {
    canvas: &'canvas mut Canvas,
    childs: Vec<Box<dyn Element>>,

    // global value
    offset: Vec2,
}

impl<'canvas> World<'canvas> {
    pub fn draw(&mut self) {
        for elem in self.childs.iter() {
            elem.as_ref().draw(&mut self.canvas);
        }
    }
}
