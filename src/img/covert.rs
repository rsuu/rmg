use image;

// Adjusted output write loop
pub fn rgba8_to_rgb8(
    input: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let width = input.width() as usize;
    let height = input.height() as usize;

    // Allocate a new buffer for the RGB image, 3 bytes per pixel
    let mut output_data = vec![0u8; width * height * 3];

    // Iterate through 4-byte chunks of the image data (RGBA bytes)
    for (output, chunk) in {
        output_data
            .chunks_exact_mut(3)
            .zip(input.as_raw().chunks_exact(4))
    } {
        // ... and copy each of them to output, leaving out the A byte
        output.copy_from_slice(&chunk[0..3]);
    }

    // Construct a new image
    image::ImageBuffer::from_raw(width as u32, height as u32, output_data).unwrap()
}

// Mostly equivalent to what happens within `image`.
pub fn convert(img: image::RgbaImage) -> image::RgbImage {
    let (width, height) = img.dimensions();
    let mut buffer: image::RgbImage = image::ImageBuffer::new(width, height);
    for (to, &image::Rgba([r, g, b, _])) in buffer.pixels_mut().zip(img.pixels()) {
        *to = image::Rgb([r, g, b]);
    }
    buffer
}
