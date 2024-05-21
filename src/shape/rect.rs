use std::ops::Add;

use crate::*;

use self::affine::Affine;

// v1          v2
//   +--------+
//   |        |
//   |        |
//   |        |
//   +--------+
// v4          v3
//
// v1 == min
// v3 == max
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect<T = Vec2> {
    pub min: T,
    pub max: T,
}

impl Rect {
    pub fn new(origin: Vec2, size: Size) -> Self {
        let w = size.width();
        let h = size.height();

        let min = origin;
        let max = Vec2::new(min.x + w, min.y + h);

        Self { min, max }
    }

    pub fn new_at_zero(size: Size) -> Self {
        let min = Vec2::new(0.0, 0.0);
        let max = Vec2::new(size.width(), size.height());

        Self { min, max }
    }

    pub fn from_center(center: Vec2, size: Size) -> Self {
        let half_width = size.width() / 2.0;
        let half_height = size.height() / 2.0;

        let center_x = center.x;
        let center_y = center.y;

        let min = Vec2::new(center_x - half_width, center_y - half_height);
        let max = Vec2::new(center_x + half_width, center_y + half_height);

        Self { min, max }
    }

    pub fn origin(&self) -> Vec2 {
        self.min
    }

    pub fn min(&self) -> Vec2 {
        self.min
    }

    pub fn max(&self) -> Vec2 {
        self.max
    }

    pub fn v1(&self) -> Vec2 {
        self.min
    }

    pub fn v2(&self) -> Vec2 {
        Vec2::new(self.max.x, self.min.y)
    }

    pub fn v3(&self) -> Vec2 {
        self.max
    }

    pub fn v4(&self) -> Vec2 {
        Vec2::new(self.min.x, self.max.y)
    }

    pub fn drag(&self, offset: Vec2) -> Self {
        let Self { min, max } = *self;

        Self {
            min: min + offset,
            max: max + offset,
        }
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
        // y
        ray.y > max.y || ray.y < min.y
    }

    pub fn is_hover(&self, other: &Self) -> bool {
        self.is_include(other.v1())
            || self.is_include(other.v2())
            || self.is_include(other.v3())
            || self.is_include(other.v4())
    }

    pub fn not_hover(&self, other: &Self) -> bool {
        !self.is_hover(other)
    }
}

impl Affine for Rect {
    fn translate(&self, dx: f32, dy: f32) -> Self {
        Self {
            min: self.min.translate(dx, dy),
            max: self.max.translate(dx, dy),
        }
    }

    fn scale(&self, dx: f32, dy: f32) -> Self {
        Self {
            min: self.min.scale(dx, dy),
            max: self.max.scale(dx, dy),
        }
    }
}
