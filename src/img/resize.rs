use crate::color::rgba::TransRgba;
use crate::{
    img::heic,
    img::size::{MetaSize, Size, TMetaSize},
    utils::err::Res,
};
use cfg_if::cfg_if;
use fir;
use image::DynamicImage;
use std::num::NonZeroU32;

#[inline(always)]
pub fn resize_rgba8(bytes: &[u8], meta: &MetaSize<u32>, filter: &fir::FilterType) -> Res<Vec<u8>> {
    let mut src_image = fir::Image::from_vec_u8(
        NonZeroU32::new(meta.image.width).ok_or(())?,
        NonZeroU32::new(meta.image.height).ok_or(())?,
        bytes.to_vec(),
        fir::PixelType::U8x4,
    )?;
    let dst_width = NonZeroU32::new(meta.fix.width).ok_or(())?;
    let dst_height = NonZeroU32::new(meta.fix.height).ok_or(())?;

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

    Ok(dst_image.buffer().to_vec())
}

#[inline(always)]
pub fn srgb_u32(buffer: &mut Vec<u32>, bytes: &[u8]) {
    for f in (0..bytes.len()).step_by(4) {
        buffer.push(TransRgba::argb_to_u32(&bytes[f..f + 4].try_into().unwrap()));
    }
}
