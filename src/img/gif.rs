// TODO

use image::codecs::gif::{GifDecoder, GifEncoder};
use image::{AnimationDecoder, ImageDecoder};
use std::fs::File;

fn load_gif() {
    // Decode a gif into frames
    let file_in = File::open("foo.gif")?;
    let mut decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding gif");
}
