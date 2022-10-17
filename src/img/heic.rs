use libheif_rs;
use log;

pub fn load_heic(bytes: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    if let Ok(ctx) = libheif_rs::HeifContext::read_from_bytes(&bytes) {
        log::debug!("{}", bytes.len());

        let handle = ctx.primary_image_handle().unwrap();

        // Decode the image
        let src_img = handle
            .decode(
                libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::Rgb),
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
            for _x in 0..width {
                res.extend_from_slice(&[bytes[step], bytes[step + 1], bytes[step + 2]]);
                step += 3;
            }
        }

        return Some((width, height, res));
    } else {
        return None;
    }
}
