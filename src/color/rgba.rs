pub struct TransRgba {}

impl TransRgba {
    pub fn argb_to_u32(rgba: &[u8; 4]) -> u32 {
        let a = (rgba[3] as u32) << 8 * 3;
        let r = (rgba[0] as u32) << 8 * 2;
        let g = (rgba[1] as u32) << 8 * 1;
        let b = (rgba[2] as u32) << 8 * 0;

        r + g + b + a
    }

    #[inline(always)]
    pub fn rgba_to_u32(rgba: &[u8; 4]) -> u32 {
        let r = (rgba[0] as u32) << 8 * 3;
        let g = (rgba[1] as u32) << 8 * 2;
        let b = (rgba[2] as u32) << 8 * 1;
        let a = (rgba[3] as u32) << 8 * 0;

        r + g + b + a
    }

    #[inline(always)]
    pub fn rgba_from_u32(rgba: u32) -> [u8; 4] {
        let r = (rgba >> 24) & 0x0ff;
        let g = (rgba >> 16) & 0x0ff;
        let b = (rgba >> 8) & 0x0ff;
        let a = rgba & 0x0ff;

        [r as u8, g as u8, b as u8, a as u8]
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
