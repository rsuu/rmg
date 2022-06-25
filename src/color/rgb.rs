use rgb;

trait ExtRgb {
    fn as_u32(&self) -> u32;
}

impl ExtRgb for rgb::RGB8 {
    #[inline(always)]
    fn as_u32(&self) -> u32 {
        let r = (self.r as u32) << 16;
        let g = (self.g as u32) << 8;
        let b = self.b as u32;

        r + g + b
    }
}

pub struct TransRgb {}

impl TransRgb {
    #[inline(always)]
    pub fn rgb_to_u32(rgb: &[u8; 3]) -> u32 {
        let r = (rgb[0] as u32) << 16;
        let g = (rgb[1] as u32) << 8;
        let b = rgb[2] as u32;

        r + g + b
    }

    #[inline(always)]
    pub fn rgb_from_u32(rgb: u32) -> [u8; 3] {
        let r = (rgb >> 16) & 0x0ff;
        let g = (rgb >> 8) & 0x0ff;
        let b = rgb & 0x0ff;

        [r as u8, g as u8, b as u8]
    }

    #[inline(always)]
    pub fn rgb_to_gray(rgb: &[u8; 3]) -> u8 {
        let r = rgb[0] as u32;
        let g = rgb[1] as u32;
        let b = rgb[2] as u32;

        ((r * 38 + g * 75 + b * 15) >> 7) as u8
    }

    #[inline(always)]
    pub fn u32_to_gray(rgb: u32) -> u8 {
        let rgb = Self::rgb_from_u32(rgb);
        Self::rgb_to_gray(&rgb)
    }
}
