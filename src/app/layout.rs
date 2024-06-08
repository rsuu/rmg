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
    Vertical {
        align: Align,
    },

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
    Horizontal {
        align: Align,
    },

    //
    //  +------------------+
    //  |                  |
    //  |                  |
    //  |        P1        |
    //  |                  |
    //  |                  |
    //  +------------------+
    //
    Single {
        flag_scroll: bool, // TODO: to App.eventinfo
        mouse_pos: Vec2,   // mv App
        /// zoom in : 1.0
        /// zoom out: -1.0
        dire: f32,
        cur_zoom: i32,
        max_zoom: i32,
        min_zoom: i32,
    },

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
    Double {
        align: Align,
        gap: Gap,
    },

    Multi {
        cols: usize,
    },

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
    Masonry {
        cols: u32,
        gap: Gap,
    },

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
    Grid {
        cols: u32,
    },
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

impl Gap {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Align {
    pub fn padding_x(&self, canvas_w: f32, elem_w: f32) -> f32 {
        // FIXME:
        match self {
            Self::Center => center_x(canvas_w, elem_w),
            Self::Left => 0.0,
            Self::Right => canvas_w - elem_w,
            _ => unreachable!(),
        }
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

pub fn center_x(canvas_w: f32, elem_w: f32) -> f32 {
    (canvas_w - elem_w) / 2.0
}

pub fn center_y(canvas_h: f32, elem_h: f32) -> f32 {
    (canvas_h - elem_h) / 2.0
}

pub fn center_xy(canvas: Size, elem: Size) -> Vec2 {
    let canvas_x = canvas.width() / 2.0;
    let canvas_y = canvas.height() / 2.0;

    let elem_x = elem.width() / 2.0;
    let elem_y = elem.height() / 2.0;

    Vec2::new(canvas_x - elem_x, canvas_y - elem_y)
}

// const THUMBNAIL_SIZE_X: usize = 100; // Assuming THUMBNAIL_SIZE_X and THUMBNAIL_SIZE_Y are constants
//const THUMBNAIL_SIZE_Y: usize = 100;
//
//pub fn padding() {
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
// pub fn center_crop() {
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
