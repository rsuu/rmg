use crate::Size;

pub fn load_heic(_bytes: &[u8]) -> eyre::Result<(Size, Vec<u8>)> {
    #[cfg(feature = "de_heic")]
    {
        return feat::load_heic(_bytes);
    }

    eyre::bail!("")
}

#[cfg(feature = "de_heic")]
mod feat {
    use crate::Size;
    use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};

    #[inline]
    pub fn load_heic(bytes: &[u8]) -> eyre::Result<(Size, Vec<u8>)> {
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
        let mut frame: Vec<u8> = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            let mut index = y as usize * stride;

            for _ in 0..width {
                frame.extend_from_slice(&bytes[index..index + 4]);
                index += 4;
            }
        }

        Ok((Size::from_u32(width, height), frame))
    }
}
