use crate::*;

use esyn::EsynDe;

#[derive(Debug, Clone, Copy, EsynDe, PartialEq)]
pub enum Layout {
    //
    //  +------------------+
    //  |                  |
    //  |                  |
    //  |        P1        |
    //  |                  |
    //  |                  |
    //  +------------------+
    //  | .  .  .  .  .  . |
    //  | .  .  .  .  .  . |
    //  | .  .   P2   .  . |
    //  | .  .  .  .  .  . |
    //  +------------------+
    //
    Vertical { align: Align },

    //
    //               (right to left)
    //  +--------+------+----+
    //  |        |      |    |
    //  |        |      |    |
    //  |   P1   |  P2  | .. |
    //  |        |      |    |
    //  |        |      |    |
    //  +--------+------+----+
    // (left to right)
    //
    Horizontal { align: Align },

    //
    //  +------------------+
    //  |                  |
    //  |                  |
    //  |        P1        |
    //  |                  |
    //  |                  |
    //  +------------------+
    //
    Single { zoom: f32, mouse_pos: Vec2 },

    //
    //  +--------+--------+
    //  |        |        |
    //  |   P1   |   P2   |
    //  |        |        |
    //  +--------+--------+
    //  |                 |
    //  |       P3        |
    //  |                 |
    //  +--------+--------+
    //  |        |        |
    //  |   P4   |   ..   |
    //  |        |        |
    //  +--------+--------+
    //
    Double { align: Align, gap: Gap },

    Multi { nums: usize },

    Gallery,

    //
    // fixed width
    //
    //  +---------+---------+
    //  |         |         |
    //  |         |   P2    |
    //  |   P1    |         |
    //  |         +---------+
    //  |         |         |
    //  +---------+         |
    //  |         |         |
    //  |   P3    |   P4    |
    //  |         |         |
    //  +---------+         |
    //  |         |         |
    //  |   P5    +---------+
    //  |         |   ..    |
    //  +---------+---------+
    //
    // ===========================
    // fixed height
    //
    //  +------+------------+
    //  |      |            |
    //  |  P1  |     P2     |
    //  |      |            |
    //  +------+-----+------+
    //  |            |      |
    //  |            |      |
    //  |     P3     |  P4  |
    //  |            |      |
    //  |            |      |
    //  +---------+--+------+
    //  |         |         |
    //  |   P5    |   ...   |
    //  |         |         |
    //  +---------+---------+
    //
    Masonry { cols: u32, gap: Gap },

    //
    //  +--------+--------+
    //  |        |        |
    //  |   P1   |   P2   |
    //  |        |        |
    //  +--------+--------+
    //  |        |        |
    //  |   P3   |   P4   |
    //  |        |        |
    //  +--------+--------+
    //  |        |        |
    //  |   P5   |   ..   |
    //  |        |        |
    //  +--------+--------+
    //
    Grid { cols: u32 },
    //  +------+------------+------------+------+
    //  |      |            |            |      |
    //  |  P1  |     P2     |     P5     |  P6  |
    //  |      |            |            |      |
    //  +------+-----+------+----------+-+------+
    //  |            |      |          |        |
    //  |     P3     |  P4  |    P7    |   P8   |
    //  |            |      |          |        |
    //  +------------+------+----------+--------+
    //
    //  +----------+----------+----------+
    //  |          |          |          |
    //  |    P1    |          |    P4    |
    //  |          |          +----------+
    //  +----------+    P3    |          |
    //  |          |          |    P5    |
    //  |    P2    |          |          |
    //  |          |          |          |
    //  +----------+----------+----------+
    //
}

#[derive(Debug, Default, Clone, Copy, EsynDe, PartialEq)]
pub struct Gap<T = f32> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Default, Clone, Copy, EsynDe, PartialEq)]
pub enum Align {
    #[default]
    Center,

    Left,
    Right,
}

impl Gap {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Layout {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical { .. } => "VERTICAL",
            Self::Horizontal { .. } => "HORIZONTAL",
            Self::Double { .. } => "DOUBLE",
            Self::Multi { .. } => "MULTI",
            Self::Masonry { .. } => "MASONRY",
            _ => todo!(),
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::Vertical {
            align: Align::default(),
        }
    }
}

// const THUMBNAIL_SIZE_X: usize = 100; // Assuming THUMBNAIL_SIZE_X and THUMBNAIL_SIZE_Y are constants
//const THUMBNAIL_SIZE_Y: usize = 100;
//
//fn padding() {
//    let src_size_x = 50; // Sample value for $src_size_x
//    let src_size_y = 60; // Sample value for $src_size_y
//
//    let mut dst_size_x: usize;
//    let mut dst_size_y: usize;
//
//    if src_size_x <= THUMBNAIL_SIZE_X && src_size_y <= THUMBNAIL_SIZE_Y {
//        dst_size_x = src_size_x;
//        dst_size_y = src_size_y;
//    } else {
//        dst_size_x = THUMBNAIL_SIZE_X;
//        dst_size_y = (src_size_y * THUMBNAIL_SIZE_X / src_size_x) as usize;
//
//        if dst_size_y > THUMBNAIL_SIZE_Y {
//            dst_size_x = (src_size_x * THUMBNAIL_SIZE_Y / src_size_y) as usize;
//            dst_size_y = THUMBNAIL_SIZE_Y;
//        }
//    }
//
//    // Printing the result
//    println!("dst_size_x: {}, dst_size_y: {}", dst_size_x, dst_size_y);
//}
//
// fn center_crop() {
// let mut src_h: i32 = src_size_y;
// let mut src_w: i32 = (src_h as f64 * THUMBNAIL_SIZE_X as f64 / THUMBNAIL_SIZE_Y as f64) as i32;
//
// if src_w <= src_size_x {
//     let src_x = (src_size_x - src_w) / 2;
//     let src_y = 0;
// } else {
//     src_w = src_size_x;
//     src_h = (src_w as f64 * THUMBNAIL_SIZE_Y as f64 / THUMBNAIL_SIZE_X as f64) as i32;
//     let src_x = 0;
//     let src_y = (src_size_y - src_h) / 2;
// }
// }
