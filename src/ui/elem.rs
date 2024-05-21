use crate::*;

use rgb::RGBA8;

#[derive(Debug)]
pub struct Nav {
    pub dire: Direction,
}

#[derive(Debug, Default)]
pub struct LeaveBlank {
    pub rect: Rect,
    pub size: Size,
    pub bg: RGBA8,
}

#[derive(Debug)]
pub enum Direction {
    Top,
    Right,
    Buttom,
    Left,
}

impl Nav {
    pub fn new(dire: Direction) -> Self {
        Self { dire }
    }
}

impl Element for LeaveBlank {
    fn draw(&self, canvas: &mut Canvas) {
        let w = self.size.width() as i32;
        let h = self.size.height() as i32;

        for y in 0..h as i32 {
            let mut index = w * y;

            for x in 0..w as i32 {
                index += x;

                todo!()
                // body.canvas.buffer[index] =
                //
                //     u32::from_be_bytes([self.bg.a, self.bg.r, self.bg.g, self.bg.b]);
            }
        }

        canvas.offset.x = 0.0;
        canvas.offset.y += h as f32;
    }

    fn empty() -> Self
    where
        Self: Sized,
    {
        Default::default()
    }
}
