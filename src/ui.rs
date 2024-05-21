pub mod elem;
pub mod style;

use crate::*;

// pub type Elems = Vec<Box<dyn Element>>;

pub trait Element {
    fn empty() -> Self
    where
        Self: Sized;

    fn draw(&self, canvas: &mut Canvas);
}

pub struct World {
    elems: Vec<Box<dyn Element>>,

    // global value
    offset: Vec2,
}

impl World {
    pub fn draw(&mut self, canvas: &mut Canvas) {
        for elem in self.elems.iter() {
            elem.as_ref().draw(canvas);
        }
    }
}
