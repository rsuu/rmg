use crate::*;

use fir::ResizeAlg;
use imagesize::blob_size;
use std::ops::{Add, Neg};

pub type Pages = Vec<Page>;

// ?Page.size and Frame.size
#[derive(Default, Clone)]
pub struct Page {
    pub frame: Frame,
    pub dst_size: Size,
    pub cast_vertex: RectVertex,
    pub offset: Vec2,

    pub state: State,
    pub index: usize,
    pub style: Style,

    pub tmp_blob: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum State {
    #[default]
    Empty,

    Waiting,
    Done,
}

// v1          v2
//   +--------+
//   |        |
//   |        |
//   |        |
//   +--------+
// v4          v3
#[derive(Debug, Clone, Copy, Default)]
pub struct RectVertex<T = Vec2> {
    pub v1: T,
    pub v2: T,
    pub v3: T,
    pub v4: T,
}

impl Page {
    pub fn draw(&mut self, buffer: &mut Buffer, size: Size) {
        let cast_v = self.cast_vertex;
        let cast_w = self.frame.size.width() as i32;
        let cast_h = self.frame.size.height() as i32;
        let (cw, ch) = (size.width() as i32, size.height() as i32);

        let (min_x, min_y, max_x, max_y) = {
            let min_x = cast_v.v1.x as i32;
            let min_y = cast_v.v1.y as i32;

            let max_x = cast_v.v3.x as i32;
            let max_y = cast_v.v3.y as i32;

            (
                min_x.clamp(0, cw),
                min_y.clamp(0, ch),
                max_x.clamp(0, cw),
                max_y.clamp(0, ch),
            )
        };

        // FIXME: this will skip first frame in gif
        let frame = self.frame.next_frame().to_argb_bytes();

        for y in min_y..max_y {
            for x in min_x..max_x {
                // let index = (size.width() as i32 * y + x) as usize;

                let dst = {
                    // TODO: ?rotate
                    // let c = self.centroid();
                    // let p = Vec2::new(x as f32, y as f32);
                    // let r = 90.0_f32;
                    //
                    // let (x, y) = rotate_at(c, p, r);
                    // let x = x.clamp(0, w);
                    // let y = y.clamp(0, h);

                    (cw * y + x) as usize
                };
                let src = {
                    let v = cast_v.v1;
                    let x = x - v.x as i32;
                    let y = y - v.y as i32;

                    (cast_w * y + x) as usize
                };

                buffer[dst] = frame[src];
            }
        }
    }

    pub fn new_empty(index: usize) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }

    pub fn set_size(&mut self, data: &DataType, flag_cache: bool) -> eyre::Result<()> {
        let blob = data.get_file(self.index)?;

        // let ty = image_type(blob.as_slice())?;
        let size = blob_size(blob.as_slice())?;
        let w = size.width as f32;
        let h = size.height as f32;
        let size = Size::new(w, h);

        self.frame.size = size;

        if flag_cache {
            self.tmp_blob = blob;
        }

        Ok(())
    }

    pub fn resize(&mut self, data: &DataType, algo: ResizeAlg) -> eyre::Result<()> {
        if self.tmp_blob.is_empty() {
            self.tmp_blob = data.get_file(self.index)?;
        }

        let frame = Frame::resize(self.tmp_blob.as_slice(), self.dst_size, algo)?;

        self.frame = frame;
        self.cast_vertex = self.frame.vertex;
        self.state = State::Done;

        self.tmp_blob.clear();
        self.tmp_blob.shrink_to(0);

        Ok(())
    }

    pub fn free(&mut self) {
        self.frame.free();
        self.state = State::Empty;
    }

    pub fn flip(&mut self) {
        self.frame.flip();
    }

    pub fn drag(&mut self, offset: Vec2) {
        let new = self.frame.vertex.add_offset(offset);

        self.cast_vertex = new;
    }

    pub fn centroid(&self) -> Vec2 {
        let Vec2 { x, y } = self.cast_vertex.v1;
        let Size { width, height } = self.frame.size;

        Vec2::new(x + width as f32 / 2.0, y + height as f32 / 2.0)
    }

    pub fn page_unmber(&self) -> usize {
        self.index + 1
    }

    pub fn is_outside(&self, ray: Vec2) -> bool {
        let v1 = self.frame.vertex().v1;
        let v3 = self.frame.vertex().v3;

        ray.x < v1.x || ray.x > v3.x || ray.y < v1.y || ray.y > v3.y
    }

    pub fn is_passed(&mut self, config: &Config, cur_view: &CurView) -> bool {
        let (is_page_at_border, is_page_at_view) = cur_view.is_page_inside(&self);

        // dbg!(cur_view);
        match (&mut self.state, is_page_at_border, is_page_at_view) {
            // 2. drawing
            (State::Done, true, ..) => true,
            // 3. freed
            (State::Done, false, false) => {
                self.free();
                self.state = State::Empty;

                false
            }

            // 1.1 waiting
            (State::Waiting, ..) => true,

            // 1. loading
            // 1.1. set_size()
            // 1.2. resize()
            (State::Empty, true, ..) => {
                self.state = State::Waiting;

                true
            }

            _ => false,
        }
    }
}

impl RectVertex {
    pub fn new(origin: Vec2, size: Size) -> Self {
        let w = size.width();
        let h = size.height();

        let x = origin.x;
        let y = origin.y;
        let v1 = Vec2::new(x, y);
        let v2 = Vec2::new(w, y);
        let v3 = Vec2::new(w, h);
        let v4 = Vec2::new(x, h);

        Self { v1, v2, v3, v4 }
    }

    pub fn min(&self) -> Vec2 {
        self.v1
    }

    pub fn max(&self) -> Vec2 {
        self.v3
    }

    pub fn add_offset(&self, offset: Vec2) -> Self {
        let Self { v1, v2, v3, v4 } = *self;

        let v1 = v1 + offset;
        let v2 = v2 + offset;
        let v3 = v3 + offset;
        let v4 = v4 + offset;

        Self { v1, v2, v3, v4 }
    }

    pub fn sub_offset(&self, offset: Vec2) -> Self {
        let Self { v1, v2, v3, v4 } = *self;

        let v1 = v1 - offset;
        let v2 = v2 - offset;
        let v3 = v3 - offset;
        let v4 = v4 - offset;

        Self { v1, v2, v3, v4 }
    }

    pub fn from_size(size: Size) -> Self {
        let w = size.width();
        let h = size.height();

        let x = 0.0;
        let y = 0.0;
        let v1 = Vec2::new(x, y);
        let v2 = Vec2::new(w, y);
        let v3 = Vec2::new(w, h);
        let v4 = Vec2::new(x, h);

        Self { v1, v2, v3, v4 }
    }

    // REFS: https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection
    pub fn is_include(&self, ray: Vec2) -> bool {
        !self.not_include(ray)
    }

    pub fn not_include(&self, ray: Vec2) -> bool {
        let min = self.min();
        let max = self.max();

        // x
        ray.x > max.x || ray.x < min.x
            ||
        // xy
        ray.y > max.y || ray.y < min.y
    }
}
