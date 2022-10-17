use crate::utils::err::{MyErr, Res};
use log;

pub fn load_heic(bytes: &[u8]) -> Res<(u32, u32, Vec<u8>)> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "de_heic")] {
            feat::load_heic(bytes)
        } else {
            Err( MyErr::FeatHeic)
        }
    }
}

#[cfg(feature = "de_heic")]
mod feat {
    use crate::utils::err::{MyErr, Res};
    use libheif_rs;

    #[inline]
    pub fn load_heic(bytes: &[u8]) -> Res<(u32, u32, Vec<u8>)> {
        if let Ok(ctx) = libheif_rs::HeifContext::read_from_bytes(&bytes) {
            log::debug!("{}", bytes.len());

            let handle = ctx.primary_image_handle().unwrap();

            // Decode the image
            let src_img = handle
                .decode(
                    libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgba),
                    false,
                )
                .unwrap();

            // pixels
            let bytes = src_img.planes().interleaved.unwrap().data;
            let width = src_img.planes().interleaved.unwrap().width;
            let height = src_img.planes().interleaved.unwrap().height;

            log::debug!("w: {} -- h: {}", width, height);
            log::debug!("{}", width * height * 3);
            log::debug!(
                "{} -- {}",
                bytes.len(),
                src_img.planes().interleaved.unwrap().stride,
            );

            let mut res: Vec<u8> = Vec::new();

            // rgb
            for y in 0..height {
                let mut step = y as usize * src_img.planes().interleaved.unwrap().stride;

                for _ in 0..width {
                    res.extend_from_slice(&[
                        bytes[step + 0],
                        bytes[step + 1],
                        bytes[step + 2],
                        bytes[step + 3],
                    ]);
                    step += 4;
                }
            }

            return Ok((width, height, res));
        } else {
            return Err(MyErr::Null(()));
        }
    }
}
