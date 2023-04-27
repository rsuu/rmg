// use image::gif::Encoder as GifEncoder;
// use image::io::Reader as ImageReader;
// use image::png::Encoder as PngEncoder;
// use image::AnimationDecoder;
//
// fn convert_gif_to_apng(gif_file: &str, apng_file: &str) -> Result<(), std::io::Error> {
//     // Read the GIF image
//     let gif_reader = ImageReader::open(gif_file)?;
//     let gif_frames = gif_reader.into_frames();
//
//     // Create an APNG encoder
//     let mut apng_encoder = PngEncoder::new(std::fs::File::create(apng_file)?);
//     apng_encoder.set_apng(true);
//
//     // Iterate over the frames of the GIF image and encode them as APNG frames
//     for gif_frame in gif_frames {
//         let mut apng_frame = gif_frame?.into_rgba8();
//         apng_frame.delay = 10; // adjust the delay time as needed
//         apng_encoder.write_frame(&apng_frame)?;
//     }
//
//     Ok(())
// }
