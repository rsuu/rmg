pub fn load_heic(_bytes: &[u8]) -> anyhow::Result<(u32, u32, Vec<Vec<u8>>)> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "de_heic")] {
            feat::load_heic(_bytes)
        } else {
            Err( anyhow::anyhow!(""))
        }
    }
}

#[cfg(feature = "de_heic")]
mod feat {
    use libheif_rs;

    #[inline]
    pub fn load_heic(bytes: &[u8]) -> anyhow::Result<(u32, u32, Vec<Vec<u8>>)> {
        if let Ok(ctx) = libheif_rs::HeifContext::read_from_bytes(bytes) {
            //tracing::debug!("{}", bytes.len());

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

            //tracing::debug!("w: {} -- h: {}", width, height);
            //tracing::debug!("{}", width * height * 3);
            //tracing::debug!(
            //    "{} -- {}",
            //    bytes.len(),
            //    src_img.planes().interleaved.unwrap().stride,
            //);

            let mut res: Vec<u8> = Vec::new();

            // rgb
            for y in 0..height {
                let mut step = y as usize * src_img.planes().interleaved.unwrap().stride;

                for _ in 0..width {
                    res.extend_from_slice(&[
                        bytes[step],
                        bytes[step + 1],
                        bytes[step + 2],
                        bytes[step + 3],
                    ]);
                    step += 4;
                }
            }

            Ok((width, height, vec![res]))
        } else {
            Err(anyhow::anyhow!(""))
        }
    }
}
