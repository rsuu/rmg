use crate::{
    color::{format::PixelFormat},
    img::size::{MetaSize, Size, TMetaSize},
    math::arrmatrix::{Affine},
    utils::types::MyResult,
};
use fast_image_resize as fir;
use image::{self, io::Reader as ImageReader};

use std::{num::NonZeroU32};

pub async fn resize(
    buffer: &mut Vec<u8>,
    path: &str,
    screen_size: Size<u32>,
    window_size: Size<u32>,
    format: PixelFormat,
) -> MyResult {
    let img = ImageReader::open(path)?.decode()?;
    // Note
    // Error: "first two bytes is not a SOI marker"
    //   You will be get a bug if the format of image is not same as its suffix
    //   e.g.
    //     format: png
    //     filename: 1.jpg
    //     return Error
    //

    let mut meta = MetaSize::<u32>::new(
        screen_size.width,
        screen_size.height,
        window_size.width,
        window_size.height,
        img.width(),
        img.height(),
    );

    meta.resize();

    //eprintln!("Open: {}", path);
    //eprintln!("Meta: {:#?}", meta);

    // BUG
    match format {
        PixelFormat::Rgb8 => resize_rgb8(buffer, &img, &meta)?,
        PixelFormat::Rgba8 => resize_rgba8(buffer, &img, &meta)?,
    }

    Ok(())
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

    // Fix BUG
    // ISSUES: https://github.com/Cykooz/fast_image_resize/issues/9
    if meta.fix.width == meta.image.width && meta.fix.height == meta.image.height {
        (*buffer).extend_from_slice(src_image.buffer()); // update buffer
    } else {
        let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
        let mut dst_view = dst_image.view_mut();
        let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Box));

        resizer.resize(&src_image.view(), &mut dst_view)?;

        (*buffer).extend_from_slice(dst_image.buffer()); // update buffer
    }

    //eprintln!("src_slice: {:?}", &src_image.buffer()[0..10]);
    //eprintln!("dst_slice: {:?}", &dst_image.buffer()[0..10]);
    //eprintln!("src_image: {}x{}", src_image.width(), src_image.height());
    //eprintln!("dst_image: {}x{}", dst_image.width(), dst_image.height());

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

    if meta.fix.width == meta.image.width && meta.fix.height == meta.image.height {
        (*buffer).extend_from_slice(src_image.buffer()); // update buffer
    } else {
        let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
        let mut dst_view = dst_image.view_mut();
        let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Box));

        resizer.resize(&src_image.view(), &mut dst_view)?;
        alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;
        alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

        (*buffer).extend_from_slice(dst_image.buffer()); // update buffer
    }

    //eprintln!("src_image: {}x{}", src_image.width(), src_image.height());
    //eprintln!("dst_image: {}x{}", dst_image.width(), dst_image.height());

    Ok(())
}
