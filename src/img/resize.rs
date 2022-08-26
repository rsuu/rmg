use crate::{
    color::format::PixelFormat,
    img::size::{MetaSize, Size, TMetaSize},
    utils::types::MyResult,
};
use fast_image_resize as fir;
use image::{self};
use log::{debug, error};
use std::num::NonZeroU32;

pub fn resize_bytes(
    bytes: &[u8],
    buffer: &mut Vec<u8>,
    screen_size: Size<u32>,
    window_size: Size<u32>,
) {
    match image::load_from_memory(&bytes) {
        Ok(ref img) => {
            let mut meta = MetaSize::<u32>::new(
                screen_size.width,
                screen_size.height,
                window_size.width,
                window_size.height,
                img.width(),
                img.height(),
            );

            meta.resize();

            resize_rgb8(buffer, img, &meta).unwrap();
        }
        Err(_) => {
            error!("Can not resize image");
        }
    }
}

pub fn resize_rgb8(
    buffer: &mut Vec<u8>,
    img: &image::DynamicImage,
    meta: &MetaSize<u32>,
) -> MyResult {
    let src_image = fir::Image::from_vec_u8(
        NonZeroU32::new(meta.image.width).ok_or(())?,
        NonZeroU32::new(meta.image.height).ok_or(())?,
        img.to_rgb8().into_raw(),
        fir::PixelType::U8x3,
    )?;
    let dst_width = NonZeroU32::new(meta.fix.width).ok_or(())?;
    let dst_height = NonZeroU32::new(meta.fix.height).ok_or(())?;

    // FIXED: https://github.com/Cykooz/fast_image_resize/issues/9
    let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Box));

    resizer.resize(&src_image.view(), &mut dst_view)?;

    (*buffer).extend_from_slice(dst_image.buffer()); // update buffer

    Ok(())
}

pub fn resize_rgba8(
    buffer: &mut Vec<u8>,
    img: &image::DynamicImage,
    meta: &MetaSize<u32>,
) -> MyResult {
    let mut src_image = fir::Image::from_vec_u8(
        NonZeroU32::new(meta.image.width).ok_or(())?,
        NonZeroU32::new(meta.image.height).ok_or(())?,
        img.to_rgba8().into_raw(),
        fir::PixelType::U8x4,
    )?;
    let dst_width = NonZeroU32::new(meta.fix.width).ok_or(())?;
    let dst_height = NonZeroU32::new(meta.fix.height).ok_or(())?;
    let alpha_mul_div = fir::MulDiv::default();

    let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Box));

    resizer.resize(&src_image.view(), &mut dst_view)?;
    alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;
    alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

    (*buffer).extend_from_slice(dst_image.buffer()); // update buffer

    Ok(())
}
