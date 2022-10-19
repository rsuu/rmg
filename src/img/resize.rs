use crate::{
    img::size::{MetaSize, Size, TMetaSize},
    utils::err::Res,
};
use cfg_if::cfg_if;
use fir;
use std::num::NonZeroU32;

pub fn resize_bytes(
    bytes: &[u8],
    buffer: &mut Vec<u8>,
    screen_size: Size<u32>,
    window_size: Size<u32>,
    filter: &fir::FilterType,
) {
    let mut meta = MetaSize::<u32>::new(
        screen_size.width,
        screen_size.height,
        window_size.width,
        window_size.height,
        0,
        0,
    );

    match image::load_from_memory(bytes) {
        Ok(ref img) => {
            meta.image.width = img.width();
            meta.image.height = img.height();
            meta.resize();

            resize_rgba8(buffer, img.to_rgba8().into_vec(), &meta, filter).unwrap();
        }

        Err(_) => {
            cfg_if! {
                // decode heic
                if #[cfg(feature="de_heic")] {
                    use crate::img::heic;

                    if let Some(res) = heic::load_heic(bytes) {
                        meta.image.width = res.0;
                        meta.image.height = res.1;
                        meta.resize();

                        resize_rgba8(buffer, res.2, &meta,filter).unwrap();

                        log::debug!("{:?}",(res.0,res.1));
                        log::debug!("{:?}",&meta);
                    }else{
                    }
                } else {}
            }
        }
    }
}

pub fn resize_rgba8(
    buffer: &mut Vec<u8>,
    bytes: Vec<u8>,
    meta: &MetaSize<u32>,
    filter: &fir::FilterType,
) -> Res<()> {
    let mut src_image = fir::Image::from_vec_u8(
        NonZeroU32::new(meta.image.width).ok_or(())?,
        NonZeroU32::new(meta.image.height).ok_or(())?,
        bytes,
        fir::PixelType::U8x4,
    )?;
    let dst_width = NonZeroU32::new(meta.fix.width).ok_or(())?;
    let dst_height = NonZeroU32::new(meta.fix.height).ok_or(())?;

    // FIXED: https://github.com/Cykooz/fast_image_resize/issues/9
    let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(*filter));

    resizer.resize(&src_image.view(), &mut dst_view)?;

    // rgba
    let alpha_mul_div = fir::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;
    alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

    (*buffer).extend_from_slice(dst_image.buffer()); // update buffer

    Ok(())
}
