use crate::*;

pub trait Affine {
    fn translate(&self, dx: f32, dy: f32) -> Self;
    fn scale(&self, dx: f32, dy: f32) -> Self;
}

impl Affine for Vec2 {
    fn translate(&self, dx: f32, dy: f32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    fn scale(&self, sx: f32, sy: f32) -> Self {
        Self::new(self.x * sx, self.y * sy)
    }
}

// pub struct Affine {}
//
// impl Affine {
//     // cx/cy: center point
//     // r: [0.0..=360.0]
//     //
//     // REFS: https://upload.wikimedia.org/wikipedia/commons/2/2c/2D_affine_transformation_matrix.svg
//     fn rotate_at(origin: Vec2, p: Vec2, r: f32) -> (i32, i32) {
//         let r = r.to_radians();
//         let (ox, oy) = (origin.x, origin.y);
//         let (x, y) = (p.x, p.y);
//
//         let new_x = (x - ox) * r.cos() - (y - oy) * r.sin() + ox;
//         let new_y = (x - ox) * r.sin() + (y - oy) * r.cos() + oy;
//
//         // FIXME: sometimes panic
//         //(new_x.round() as i32, new_y.round() as i32)
//         (new_x.round() as i32 - 1, new_y.round() as i32 - 1)
//     }
// }
