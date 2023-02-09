use crate::{img::size::Size,  FPS};
use gif;
use gif_dispose;
use std::{io::Read, mem};

pub fn load_gif(bytes: impl Read) -> anyhow::Result<(Size<u32>, Vec<Vec<u8>>, Vec<u32>)> {
    let mut gif_opts = gif::DecodeOptions::new();
    gif_opts.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = gif_opts.read_info(bytes).unwrap();
    let mut screen = gif_dispose::Screen::new_decoder(&decoder);

    let size = Size {
        width: decoder.width() as u32,
        height: decoder.height() as u32,
    };

    let mut frames = vec![];
    let mut buffer_frame = Vec::new();
    let mut pts = 0;
    let mut pts_list = vec![];

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        screen.blit_frame(frame).unwrap();

        for rgba in screen.pixels.buf().iter() {
            buffer_frame.extend_from_slice(&[rgba.r, rgba.g, rgba.b, rgba.a]);
        }

        pts += FPS as u32 + frame.delay as u32;
        pts_list.push(pts);
        frames.push(mem::take(&mut buffer_frame));
    }

    Ok((size, frames, pts_list))
}

// image
// let gif_decoder = GifDecoder::new(file)?;
// let frames = gif_decoder.into_frames().collect_frames()?;
// for f in frames {
//     let delay = f.delay().numer_denom_ms().0 as u16;
//     col.add_anim_frame(f.into_buffer(), delay);
//     col.repeat = true;
// }
