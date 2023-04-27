pub fn load_heic(_bytes: &[u8]) -> anyhow::Result<(u32, u32, Vec<Vec<u8>>)> {
    #[cfg(feature = "de_heic")]
    {
        return feat::load_heic(_bytes);
    }

    anyhow::bail!("")
}

#[cfg(feature = "de_heic")]
mod feat {
    use libheif_rs::{Channel, ColorSpace, HeifContext, ItemId, LibHeif, Result, RgbChroma};

    #[inline]
    pub fn load_heic(bytes: &[u8]) -> anyhow::Result<(u32, u32, Vec<Vec<u8>>)> {
        let lib_heif = LibHeif::new();
        let ctx = HeifContext::read_from_bytes(bytes)?;
        let handle = ctx.primary_image_handle()?;
        let img = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgba), None)?;
        let planes = img.planes();
        let interleaved = planes.interleaved.unwrap();

        let bytes = interleaved.data;
        let width = interleaved.width;
        let height = interleaved.height;
        let stride = interleaved.stride;

        // rgba
        let mut res: Vec<u8> = Vec::new();
        for y in 0..height {
            let mut step = y as usize * stride;

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
    }
}
