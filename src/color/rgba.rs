pub struct TransRgba {}

impl TransRgba {
    #[inline(always)]
    pub fn rgba_as_argb_u32(r: &u8, g: &u8, b: &u8, a: &u8) -> u32 {
        // (r, g, b, a) -> (a, r, g, b) -> u32
        //  3  2  1  0      3  2  1  0
        u32::from_be_bytes([*a, *r, *g, *b])
    }

    #[inline(always)]
    pub fn rgba_as_u32(r: &u8, g: &u8, b: &u8, a: &u8) -> u32 {
        // (r, g, b, a) -> u32
        //  3  2  1  0
        u32::from_be_bytes([*r, *g, *b, *a])
    }

    #[inline(always)]
    pub fn rgba_from_u32(rgba: &u32) -> [u8; 4] {
        // u32 -> (r, g, b, a)
        //         3  2  1  0
        // [
        //     ((rgba >> 8 * 3) & 0x0ff) as u8,
        //     ((rgba >> 8 * 2) & 0x0ff) as u8,
        //     ((rgba >> 8 * 1) & 0x0ff) as u8,
        //     (rgba & 0x0ff) as u8,
        // ];

        // SAFETY:
        unsafe { std::mem::transmute::<u32, [u8; 4]>(rgba.to_be()) }
    }
}

mod test {

    #[test]
    fn _rgba_as_argb_u32() {}

    #[test]
    fn _rgba_as_u32() {
        use crate::color::rgba::TransRgba;
        assert_eq!(16909060, TransRgba::rgba_as_u32(&1, &2, &3, &4));
    }

    #[test]
    fn _rgba_from_u32() {
        use crate::color::rgba::TransRgba;
        assert_eq!([1_u8, 2, 3, 4], TransRgba::rgba_from_u32(&16909060));
    }
}
