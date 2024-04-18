use esyn::EsynDe;

#[derive(Debug, Default, EsynDe, PartialEq)]
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
    #[default]
    Vertical,

    //
    //               (right to left)
    //  +----+----+----+----+
    //  |    |    |    |    |
    //  |    |    |    |    |
    //  | P1 | P2 | P3 | .. |
    //  |    |    |    |    |
    //  |    |    |    |    |
    //  +----+----+----+----+
    // (left to right)
    //
    Horizontal,

    //
    //  +------------------+
    //  |                  |
    //  |                  |
    //  |        P1        |
    //  |                  |
    //  |                  |
    //  +------------------+
    //
    Single,

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
    Double,

    Gallery,
    SingleImage,

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
        gap_x: u32,
        gap_y: u32,
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

impl Layout {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical => "VERTICAL",
            Self::Double => "DOUBLE",
            Self::Masonry { .. } => "MASONRY",
            _ => todo!(),
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
