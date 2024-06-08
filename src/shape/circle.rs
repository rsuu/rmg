use crate::*;

use rgb::RGBA8;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    origin: Vec2,
    r: f32,
}

impl Circle {
    pub fn new(origin: Vec2, r: f32) -> Self {
        Self { origin, r }
    }

    // REFS: https://math.stackexchange.com/questions/198764/how-to-know-if-a-point-is-inside-a-circle
    pub fn is_include(&self, ray: Vec2) -> bool {
        let Vec2 { x, y } = ray - self.origin;
        let d = (x * x) + (y * y);

        d <= (self.r * self.r)
    }

    pub fn is_ray_at(&self, ray: Vec2) -> bool {
        let Vec2 { x, y } = ray - self.origin;
        let d = (x * x) + (y * y);

        d == (self.r * self.r)
    }

    pub fn diameter(&self) -> f32 {
        self.r * 2.0
    }

    pub fn radius(&self) -> f32 {
        self.r
    }

    pub fn outer_rect(&self) -> Rect {
        let Self { mut origin, r } = self;
        origin.x -= r;
        origin.y -= r;

        let d = self.diameter();
        let size = Size::new(d, d);

        Rect::new(origin, size)
    }

    pub fn inner_rect(&self) -> Rect {
        let r = self.r * 2.0_f32.sqrt();
        let inner_circle = Circle::new(self.origin, r);

        inner_circle.outer_rect()
    }

    pub fn draw(&self, buffer: &mut Buffer, fill: RGBA8) {
        tracing::trace!(func = "Circle::draw()");

        let size = buffer.size;
        let rect = self.outer_rect();

        let (cw, ch) = (size.width() as i32, size.height() as i32);
        let (min_x, min_y, max_x, max_y) = {
            let min_x = rect.min().x as i32;
            let min_y = rect.min().y as i32;

            let max_x = rect.max().x as i32;
            let max_y = rect.max().y as i32;

            (
                min_x.clamp(0, cw),
                min_y.clamp(0, ch),
                max_x.clamp(0, cw),
                max_y.clamp(0, ch),
            )
        };
        let RGBA8 { r, g, b, a } = fill;
        let fill = u32::from_be_bytes([a, r, g, b]);

        // dbg!(&max_y, &max_x, self.origin);
        // Draw if `Ray` is at the `Circle`
        for y in min_y..max_y {
            for x in min_x..max_x {
                let ray = Vec2::new(x as f32, y as f32);
                // TODO: if outer.include && inner.not_include
                if self.is_include(ray) {
                    let dst = (cw * y + x) as usize;

                    buffer.vec[dst] = fill;
                }
            }
        }
    }
}

// REFS: http://perfectionkills.com/exploring-canvas-drawing-techniques/
