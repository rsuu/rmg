use rgb::{ComponentSlice, RGBA8};

use crate::*;

pub struct Body<'canvas> {
    canvas: &'canvas mut Canvas,
    offset: Vec2,
    widgets: Vec<Box<dyn Draw>>,
}

#[derive(Debug)]
pub struct TopBar {}

#[derive(Debug)]
pub struct ButtomBar {}

#[derive(Debug)]
pub struct LeftSideBar {}

#[derive(Debug)]
pub struct RightSideBar {}

#[derive(Debug)]
pub struct Menu {}

#[derive(Debug)]
pub struct LineBreak {
    size: Size,
    bg: RGBA8,
}

impl Draw for LineBreak {
    fn draw(&self, body: &mut Body) {
        let w = self.size.width() as i32;
        let h = self.size.height() as i32;

        for y in (0..h as i32) {
            let mut index = w * y;

            for x in (0..w as i32) {
                index += x;

                todo!()
                // body.canvas.buffer[index] =
                //
                //     u32::from_be_bytes([self.bg.a, self.bg.r, self.bg.g, self.bg.b]);
            }
        }

        body.offset.x = 0.0;
        body.offset.y += h as f32;
    }
}
