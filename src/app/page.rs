use std::ops::Neg;

use crate::*;

use fir::ResizeAlg;
use imagesize::blob_size;

use self::affine::Affine;

// ?Page.size and Frame.size
#[derive(Default, Clone)]
pub struct Page {
    pub frame: Frame,
    pub dst_size: Size,
    pub cast_vertex: Rect,
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

impl Page {
    // ClipSpace -> ScreenSpace
    //       f32 -> u32
    pub fn draw(&mut self, buffer: &mut Buffer) {
        let size = buffer.size;

        // TODO: zoom * frame.size
        let cast_v = self.cast_vertex;
        let fw = self.frame.size.width() as i32;
        let (cw, ch) = (size.width() as i32, size.height() as i32);

        let (min_x, min_y, max_x, max_y) = {
            let min_x = cast_v.min().x as i32;
            let min_y = cast_v.min().y as i32;

            let max_x = cast_v.max().x as i32;
            let max_y = cast_v.max().y as i32;

            (
                min_x.clamp(0, cw),
                min_y.clamp(0, ch),
                max_x.clamp(0, cw),
                max_y.clamp(0, ch),
            )
        };

        // FIXME(gif): missed first frame
        let frame = self.frame.next_frame().to_argb_bytes();
        // let tolerance = 1;

        for y in min_y..max_y {
            for x in min_x..max_x {
                // let index = (size.width() as i32 * y + x) as usize;

                let src = {
                    let o = cast_v.origin();
                    let x = x - o.x as i32;
                    let y = y - o.y as i32;

                    (fw * y + x) as usize
                };
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

                if src < frame.len() {
                    buffer.vec[dst] = frame[src];
                }
            }
        }
    }

    pub fn new_empty(index: usize) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }

    pub fn load(&mut self, data: &DataType, flag_cache: bool) -> eyre::Result<()> {
        let blob = data.get_file(self.index)?;

        // let ty = image_type(blob.as_slice())?;
        let size = blob_size(blob.as_slice())?;
        let w = size.width as f32;
        let h = size.height as f32;
        let size = Size::new(w, h);

        self.frame.size = size;
        self.frame.vertex = Rect::new_at_zero(size);

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

    pub fn zoom_at(&mut self, at: Vec2, factor: f32) {
        let Vec2 { x, y } = at;

        self.cast_vertex = self.cast_vertex.translate(x.neg(), y.neg());
        self.cast_vertex = self.cast_vertex.scale(factor, factor);
        self.cast_vertex = self.cast_vertex.translate(x, y);
    }

    pub fn drag(&mut self, offset: Vec2) {
        let new = self.frame.vertex.drag(offset);
        self.cast_vertex = new;

        // self.cast_vertex = self.cast_vertex.drag(offset);
    }

    pub fn centroid(&self) -> Vec2 {
        let Vec2 { x, y } = self.cast_vertex.origin();
        let Size { width, height } = self.frame.size;

        Vec2::new(x + width as f32 / 2.0, y + height as f32 / 2.0)
    }

    pub fn page_unmber(&self) -> usize {
        self.index + 1
    }

    pub fn is_passed(&mut self, config: &Config, view_area: &ViewArea) -> bool {
        let (is_hover_edge, is_hover_view) = view_area.is_page_hover(&self);

        // dbg!(is_hover_edge, is_hover_view);
        match (&mut self.state, is_hover_edge, is_hover_view) {
            // 2. drawing
            (State::Done, true, ..) => true,

            // 3. free
            (State::Done, false, false) => {
                tracing::debug!(
                    action = "free",
                    index = ?self.index
                );

                self.free();
                self.state = State::Empty;

                false
            }

            // 1.2. resize()
            (State::Waiting, ..) => true,

            // 1. loading
            // 1.1. open_img()
            (State::Empty, true, ..) => {
                self.state = State::Waiting;
                // dbg!("empty");

                true
            }

            _ => false,
        }
    }
}

impl Element for Page {
    type Res = eyre::Result<()>;

    fn size(&self) -> Size {
        self.frame.size
    }

    fn draw<'a>(&self, args: &'a mut ElementArgs) -> Self::Res {
        Ok(())
    }
}
