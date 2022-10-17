use image::codecs::gif::{GifDecoder, GifEncoder};
use image::{AnimationDecoder, ImageDecoder};
use std::fs::File;

fn load_anim(bytes: &[u8]) {
    if let Ok(ref anim) = image::load_from_memory(bytes) {
        // let frames = gif::Frame::from_rgb(anim.width(), anim.height(), &mut anim.to_rgb8().to_vec());
    }
}
