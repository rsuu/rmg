pub mod align;
pub mod elem;
pub mod style;

use crate::*;

// ?match attrs
pub trait Element {
    type Res;

    fn new() -> Self
    where
        Self: Sized + Default,
    {
        Default::default()
    }

    fn draw<'a>(&self, args: &'a mut ElementArgs) -> Self::Res;
    fn size(&self) -> Size;

    // pub fn style(&self) -> &Style;
    // pub fn state(&self) -> &State;
    // pub fn frame(&self) -> &Frame;
    // pub fn predraw(&self) -> &[u8];
    // pub fn vertex(&self) -> &Rect<Vec2>;
    // pub fn next_frame(&mut self);
    // pub fn reload(&mut self);
}

pub struct ElementArgs<'a> {
    canvas: &'a mut Canvas,
}

// REFS: https://github.com/zed-industries/zed/blob/dc141d0f6182773b1281b019c28bcd97413102f0/crates/gpui/src/elements/img.rs
