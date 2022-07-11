
use std::num::NonZeroU32;


use image::io::Reader as ImageReader;


use fast_image_resize as fir;

fn main() {
    not_work(); // output: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    work(); // output: [255, 196, 181, 255, 196, 181, 255, 198, 181, 255]
}

fn work() {
    let img = ImageReader::open("/dev/shm/0/1.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();

    let src_image = fir::Image::from_vec_u8(
        width,
        height,
        img.to_rgb8().into_raw(),
        fir::PixelType::U8x3,
    )
    .unwrap();

    let fix_width = NonZeroU32::new(img.width() * 2).unwrap();
    let fix_height = NonZeroU32::new(img.height() * 2).unwrap();
    let dst_width = fix_width; // fix here
    let dst_height = fix_height;

    let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Box));

    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    eprintln!("{:?}", &dst_image.buffer()[0..10]);
}

fn not_work() {
    let img = ImageReader::open("/dev/shm/0/1.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();

    let src_image = fir::Image::from_vec_u8(
        width,
        height,
        img.to_rgb8().into_raw(),
        fir::PixelType::U8x3,
    )
    .unwrap();
    let dst_width = width; // bug here
    let dst_height = height;

    let mut dst_image = fir::Image::new(dst_width, dst_height, src_image.pixel_type());
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Box));

    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    eprintln!("{:?}", &dst_image.buffer()[0..10]);
}
