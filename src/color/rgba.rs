pub struct TransRgba {}

impl TransRgba {
    #[inline(always)]
    pub fn rgba_as_argb_u32(r: &u8, g: &u8, b: &u8, a: &u8) -> u32 {
        // (r, g, b, a) -> (a, r, g, b) -> u32
        //  3  2  1  0      3  2  1  0
        ((*r as u32) << 8 * 2)
            + ((*g as u32) << 8 * 1)
            + ((*b as u32) << 8 * 0)
            + ((*a as u32) << 8 * 3)
    }

    #[inline(always)]
    pub fn rgba_as_u32(r: &u8, g: &u8, b: &u8, a: &u8) -> u32 {
        // (r, g, b, a) -> u32
        //  3  2  1  0
        ((*r as u32) << 8 * 3)
            + ((*g as u32) << 8 * 2)
            + ((*b as u32) << 8 * 1)
            + ((*a as u32) << 8 * 0)
    }

    #[inline(always)]
    pub fn rgba_from_u32(rgba: u32) -> (u8, u8, u8, u8) {
        // u32 -> (r, g, b, a)
        //         3  2  1  0
        (
            ((rgba >> 8 * 3) & 0x0ff) as u8,
            ((rgba >> 8 * 2) & 0x0ff) as u8,
            ((rgba >> 8 * 1) & 0x0ff) as u8,
            (rgba & 0x0ff) as u8,
        )
    }
}

// trait ExtRgba {
//     fn as_u32(&self) -> u32;
// }
//
// impl ExtRgba for rgb::RGBA8 {
//     #[inline(always)]
//     fn as_u32(&self) -> u32 {
//         let r = (self.r as u32) << 24;
//         let g = (self.g as u32) << 16;
//         let b = (self.b as u32) << 8;
//         let a = self.a as u32;
//
//         r + g + b + a
//     }
// }
