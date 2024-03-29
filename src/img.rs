// TODO: ?AVIF/HEIC -> Anim

// ==============================================
// feature
pub mod ase;
pub mod avif;
pub mod gif;
pub mod heic;
pub mod svg;

// ==============================================
use esyn::*;
use fir;
use image;
use std::{mem, num::NonZeroU32};

pub struct TransRgb {}
pub struct TransRgba {}

#[derive(Debug, Clone, Copy, EsynDe)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

#[derive(Debug, Clone, Copy)]
pub struct MetaSize<T> {
    pub screen: Size<T>,
    pub window: Size<T>,
    pub image: Size<T>,
    pub fix: Size<T>,
}

#[derive(Debug, Clone, Copy, EsynDe)]
pub enum FilterType {
    Box,
    Bilinear,
    Hamming,
    CatmullRom,
    Mitchell,
    Lanczos3,
}

// ==============================================
pub trait TMetaSize {
    type T;

    fn new(sw: Self::T, sh: Self::T, ww: Self::T, wh: Self::T, iw: Self::T, ih: Self::T) -> Self;

    fn resize(&mut self);
}

// ==============================================
impl<T> Size<T> {
    // TOOD: fn resize()
    // TOOD: fn get_resize()
    pub fn new(width: T, height: T) -> Self {
        Size { width, height }
    }

    pub fn len(&self) -> usize
    where
        T: TryInto<usize> + Copy,
        T::Error: std::fmt::Debug,
    {
        self.width.try_into().unwrap() * self.height.try_into().unwrap()
    }
}

impl TransRgba {
    #[inline(always)]
    pub fn rgba_as_argb_u32(r: &u8, g: &u8, b: &u8, a: &u8) -> u32 {
        // (r, g, b, a) -> (a, r, g, b) -> u32
        //  3  2  1  0      0  3  2  1
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
        rgba.to_be().to_ne_bytes()

        // [
        //     ((rgba >> 8 * 3) & 0x0ff) as u8,
        //     ((rgba >> 8 * 2) & 0x0ff) as u8,
        //     ((rgba >> 8 * 1) & 0x0ff) as u8,
        //     (rgba & 0x0ff) as u8,
        // ];

        // SAFETY:
        //unsafe { std::mem::transmute::<u32, [u8; 4]>(rgba.to_be()) }
    }
}

// use rgb;
//
// trait ExtRgb {
//     fn as_u32(&self) -> u32;
// }
//
// impl ExtRgb for rgb::RGB8 {
//     #[inline(always)]
//     fn as_u32(&self) -> u32 {
//         let r = (self.r as u32) << 16;
//         let g = (self.g as u32) << 8;
//         let b = self.b as u32;
//
//         r + g + b
//     }
// }

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

// ==============================================
impl TMetaSize for MetaSize<u32> {
    type T = u32;

    fn new(sw: Self::T, sh: Self::T, ww: Self::T, wh: Self::T, iw: Self::T, ih: Self::T) -> Self {
        MetaSize {
            screen: Size::<u32>::new(sw, sh),
            window: Size::<u32>::new(ww, wh),
            image: Size::<u32>::new(iw, ih),
            fix: Size::<u32>::new(0, 0),
        }
    }

    fn resize(&mut self) {
        // e.g. w = 3, h = 4
        //      w = (w/2)*2 = 2
        //      h = (h/2)*2 = 4
        let q = self.image.width as f32 / self.image.height as f32;
        let w = self.window.width as f32;
        let h = w / q;

        self.fix.width = (w as Self::T / 2) * 2;
        self.fix.height = (h as Self::T / 2) * 2;
    }
}

// ==============================================
pub fn resize_rgba8(
    bytes: &mut Vec<u8>,
    from: &Size<u32>,
    to: &Size<u32>,
    filter: &FilterType,
) -> anyhow::Result<()> {
    let mut src_image = fir::Image::from_vec_u8(
        NonZeroU32::new(from.width).unwrap(),
        NonZeroU32::new(from.height).unwrap(),
        bytes.clone(),
        fir::PixelType::U8x4,
    )?;
    let dst_width = NonZeroU32::new(to.width).unwrap();
    let dst_height = NonZeroU32::new(to.height).unwrap();

    let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(filter.to_fir()));

    unsafe {
        #[cfg(feature = "arch_sse4_1")]
        {
            resizer.set_cpu_extensions(fir::CpuExtensions::Sse4_1);
        }

        #[cfg(feature = "arch_avx2")]
        {
            resizer.set_cpu_extensions(fir::CpuExtensions::Avx2);
        }
    }

    resizer.resize(&src_image.view(), &mut dst_view)?;

    // rgba
    let alpha_mul_div = fir::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;
    alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

    *bytes = mem::take(&mut dst_image.buffer().to_owned());

    Ok(())
}

#[inline]
pub fn center_img<T>(bg: &mut Vec<T>, fg: &mut Vec<T>, bgw: usize, fgw: usize, h: usize) {
    let x_offset = (bgw - fgw) / 2;

    for y in 0..h {
        let fp = fgw * y;
        let bp = bgw * y + x_offset;

        for x in 0..fgw {
            mem::swap(&mut bg[bp + x], &mut fg[fp + x]);
        }
    }
}

pub fn crop_img() {}

/// `Vec<u8> -> Vec<u32>`
#[inline]
pub fn argb_u32(buffer: &mut Vec<u32>, bytes: &[u8]) {
    *buffer = vec![0; bytes.len() / 4];

    for (idx, f) in (0..bytes.len()).step_by(4).enumerate() {
        buffer[idx] =
            TransRgba::rgba_as_argb_u32(&bytes[f], &bytes[f + 1], &bytes[f + 2], &bytes[f + 3]);
    }
}

#[inline]
pub fn rgba_u32(buffer: &mut Vec<u32>, bytes: &[u8]) {
    *buffer = vec![0; bytes.len() / 4];

    for (idx, f) in (0..bytes.len()).step_by(4).enumerate() {
        buffer[idx] =
            TransRgba::rgba_as_u32(&bytes[f], &bytes[f + 1], &bytes[f + 2], &bytes[f + 3]);
    }
}

// ==============================================
mod test {
    pub use crate::img::*;

    #[test]
    fn _rgba_as_argb_u32() {}

    #[test]
    fn _rgba_as_u32() {
        assert_eq!(16909060, TransRgba::rgba_as_u32(&1, &2, &3, &4));
    }

    #[test]
    fn _rgba_from_u32() {
        assert_eq!([1_u8, 2, 3, 4], TransRgba::rgba_from_u32(&16909060));
    }
}

impl FilterType {
    pub fn to_fir(&self) -> fir::FilterType {
        match self {
            Self::Box => fir::FilterType::Box,
            Self::Bilinear => fir::FilterType::Bilinear,
            Self::Hamming => fir::FilterType::Hamming,
            Self::CatmullRom => fir::FilterType::CatmullRom,
            Self::Mitchell => fir::FilterType::Mitchell,
            Self::Lanczos3 => fir::FilterType::Lanczos3,
            _ => unreachable!(),
        }
    }

    pub fn from_str(v: &str) -> Self {
        match v {
            "box" => Self::Box,
            "bilinear" => Self::Bilinear,
            "hamming" => Self::Hamming,
            "catmullRom" => Self::CatmullRom,
            "mitchell" => Self::Mitchell,
            "lanczos3" => Self::Lanczos3,
            _ => unreachable!(),
        }
    }
}
