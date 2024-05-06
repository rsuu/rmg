use crate::{utils::is_similar, Size};

use esyn::EsynDe;
use std::ops::{Add, AddAssign, Div, Sub};

#[derive(Debug, Default, Clone, Copy, Eq, PartialOrd, Ord, PartialEq, EsynDe)]
pub struct Vec2<T = f32> {
    pub x: T,
    pub y: T,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn len(&self) -> f32 {
        let Self { x, y } = self;

        (x * x + y * y).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.len();

        if is_similar(len, 0.0) {
            Self::new(0.0, 0.0)
        } else {
            Self::new(self.x / len, self.y / len)
        }
    }

    pub fn unnormalized(&self, size: Size) -> Self {
        let Size { width, height } = size;

        let x = (self.x + 1.0) * 0.5 * width;
        let y = (1.0 - self.y) * 0.5 * height;

        Self::new(x, y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

pub struct Affine {}

impl Affine {
    // cx/cy: center point
    // r: [0.0..=360.0]
    //
    // REFS: https://upload.wikimedia.org/wikipedia/commons/2/2c/2D_affine_transformation_matrix.svg
    fn rotate_at(origin: Vec2, p: Vec2, r: f32) -> (i32, i32) {
        let r = r.to_radians();
        let (ox, oy) = (origin.x, origin.y);
        let (x, y) = (p.x, p.y);

        let new_x = (x - ox) * r.cos() - (y - oy) * r.sin() + ox;
        let new_y = (x - ox) * r.sin() + (y - oy) * r.cos() + oy;

        // FIXME: sometimes panic
        //(new_x.round() as i32, new_y.round() as i32)
        (new_x.round() as i32 - 1, new_y.round() as i32 - 1)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
