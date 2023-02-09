use crate::{
    color::rgba::TransRgba,
    img::size::{Size, TMetaSize},
};
use cfg_if::cfg_if;
use fir;
use std::mem;
use std::num::NonZeroU32;

pub fn resize_rgba8(
    bytes: &mut Vec<u8>,
    from: &Size<u32>,
    to: &Size<u32>,
    filter: &fir::FilterType,
) -> anyhow::Result<()> {
    tracing::debug!("{:?}", from);
    tracing::debug!("{:?}", to);

    let mut src_image = fir::Image::from_vec_u8(
        NonZeroU32::new(from.width).unwrap(),
        NonZeroU32::new(from.height).unwrap(),
        bytes.clone(),
        fir::PixelType::U8x4,
    )?;
    let dst_width = NonZeroU32::new(to.width).unwrap();
    let dst_height = NonZeroU32::new(to.height).unwrap();

    // FIXED: https://github.com/Cykooz/fast_image_resize/issues/9
    let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(*filter));

    cfg_if! {
        if #[cfg(feature="sse4_1")] {
            unsafe { resizer.set_cpu_extensions(fir::CpuExtensions::Sse4_1); }
        } else if #[cfg(feature="avx2")]{
            unsafe { resizer.set_cpu_extensions(fir::CpuExtensions::Avx2); }
        } else {}
    }

    resizer.resize(&src_image.view(), &mut dst_view)?;

    // rgba
    let alpha_mul_div = fir::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;
    alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

    *bytes = mem::take(&mut dst_image.buffer().to_owned());

    Ok(())
}

pub fn center_img(bg: &mut Vec<u32>, fg: &[u32], bw: usize, fw: usize, fh: usize, offset: usize) {
    *bg = vec![0; bw * fh];

    let mut idx_fg = 0;
    let mut idx_bg = 0;

    for y in 0..fh {
        idx_fg = fw * y;
        idx_bg = (bw * y) + offset;

        for x in 0..fw {
            bg[idx_bg + x] = fg[idx_fg + x];
        }
    }
}

pub fn crop_img2(img: &[u32], offset: usize, iw: usize, ih: usize, ow: usize) -> Vec<u32> {
    let mut buffer = vec![0; ow * ih];

    let mut i = 0;
    let mut o = 0;

    for y in 0..ih {
        i = (iw * y) + offset;
        o = ow * y;

        for x in 0..ow {
            buffer[o + x] = img[i + x];
        }
    }

    buffer
}

pub fn crop_img(
    buffer: &mut Vec<u32>,
    img: &[u32],
    offset: usize,
    iw: usize,
    ih: usize,
    ow: usize,
) {
    *buffer = vec![0; ow * ih];

    let mut i = 0;
    let mut o = 0;

    for y in 0..ih {
        i = (iw * y) + offset;
        o = ow * y;

        // for x in 0..ow {
        //     buffer[o + x] = img[i + x];
        // }
        buffer[o..(ow + o)].copy_from_slice(&img[i..(ow + i)]);
    }
}

pub fn argb_u32(buffer: &mut Vec<u32>, bytes: &[u8]) {
    *buffer = vec![0; bytes.len() / 4];

    for (idx, f) in (0..bytes.len()).step_by(4).enumerate() {
        buffer[idx] =
            TransRgba::rgba_as_argb_u32(&bytes[f], &bytes[f + 1], &bytes[f + 2], &bytes[f + 3]);
    }
}

pub fn rgba_u32(buffer: &mut Vec<u32>, bytes: &[u8]) {
    *buffer = vec![0; bytes.len() / 4];

    for (idx, f) in (0..bytes.len()).step_by(4).enumerate() {
        buffer[idx] =
            TransRgba::rgba_as_u32(&bytes[f], &bytes[f + 1], &bytes[f + 2], &bytes[f + 3]);
    }
}

pub fn yuv420_u32() {}
