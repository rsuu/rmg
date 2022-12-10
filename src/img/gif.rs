use gif;


use std::io::Read;
use std::mem;

use crate::utils::err::Res;
use crate::utils::traits::AutoLog;
use gif_dispose;

use super::size::Size;

pub fn load_gif(bytes: impl Read) -> Res<(Size<u32>, Vec<Vec<u8>>)> {
    let mut res = vec![];
    let mut buffer_frame = Vec::new();

    let mut gif_opts = gif::DecodeOptions::new();
    gif_opts.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = gif_opts.read_info(bytes).unwrap();
    let mut screen = gif_dispose::Screen::new_decoder(&decoder);

    let size = Size {
        width: decoder.width() as u32,
        height: decoder.height() as u32,
    };

    loop {
        "decoded frame"._dbg();

        if let Some(frame) = decoder.read_next_frame().unwrap() {
            screen.blit_frame(frame).unwrap();

            for rgba in screen.pixels.buf().iter() {
                buffer_frame.push(rgba.r);
                buffer_frame.push(rgba.g);
                buffer_frame.push(rgba.b);
                buffer_frame.push(rgba.a);
            }

            res.push(mem::take(&mut buffer_frame));
        } else {
            break;
        }
    }

    Ok((size, res))
}

// image
// let gif_decoder = GifDecoder::new(file)?;
// let frames = gif_decoder.into_frames().collect_frames()?;
// for f in frames {
//     let delay = f.delay().numer_denom_ms().0 as u16;
//     col.add_anim_frame(f.into_buffer(), delay);
//     col.repeat = true;
// }
