fn main() {
    #[cfg(feature = "de_heic")]
    {
        // cg ex heic -F de_heic
        use libheif_rs;

        let ctx = libheif_rs::HeifContext::read_from_file("./tests/1.heic").unwrap();
        let handle = ctx.primary_image_handle().unwrap();

        let img = handle
            .decode(
                libheif_rs::ColorSpace::YCbCr(libheif_rs::Chroma::C420),
                false,
            )
            .unwrap();

        let sc = 32;
        let width = img.planes().y.unwrap().width;
        let height = img.planes().y.unwrap().height;

        let img = img.scale(width / sc * sc, height / sc * sc, None).unwrap();

        let mut bytes = img.planes().y.unwrap().data.to_vec();
        bytes.extend_from_slice(img.planes().cb.unwrap().data);
        bytes.extend_from_slice(img.planes().cr.unwrap().data);
        let width = img.planes().y.unwrap().width;
        let height = img.planes().y.unwrap().height;
        let _stride = img.planes().y.unwrap().stride;

        let res = yuvi420_to_rgb8::<u8>(&bytes, width as usize, height as usize);

        image::save_buffer("w.png", &res, width, height, image::ColorType::Rgb8).unwrap();
    }
}

fn yuvi420_to_rgb8<Yuv_>(img: &[Yuv_], w: usize, h: usize) -> Vec<u8>
where
    Yuv_: Into<f32> + Copy,
{
    let size = w * h;
    let u_pos = size;
    let v_pos = size + size / 4;

    let mut res: Vec<u8> = vec![0_u8; size * 3];

    for y in 0..h {
        let step = (y / 2) * (w / 2);
        let ys = y * w;
        let us = u_pos + step;
        let vs = v_pos + step;

        for x in 0..w {
            let y = ys + x;
            let u = us + x / 2;
            let v = vs + x / 2;
            let idx = y * 3;

            let rgb = yuv_to_rgb(img[y].into(), img[u].into(), img[v].into());

            res[idx] = rgb[0];
            res[idx + 1] = rgb[1];
            res[idx + 2] = rgb[2];
        }
    }

    res
}

#[inline(always)]
fn yuv_to_rgb(y: f32, u: f32, v: f32) -> [u8; 3] {
    let mut r: f32 = y + 1.402 * (v - 128.0);
    let mut g: f32 = y - 0.34414 * (u - 128.0) - 0.71414 * (v - 128.0);
    let mut b: f32 = y + 1.772 * (u - 128.0);

    if r < 0.0 {
        r = 0.0;
    }
    if g < 0.0 {
        g = 0.0;
    }
    if b < 0.0 {
        b = 0.0;
    }
    if r > 255.0 {
        r = 255.0;
    }
    if g > 255.0 {
        g = 255.0;
    }
    if b > 255.0 {
        b = 255.0;
    }

    [r as u8, g as u8, b as u8]
}
