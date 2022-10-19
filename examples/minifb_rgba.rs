
use image::io::Reader as ImageReader;

use minifb::{Scale, ScaleMode, Window};

fn main() {
    // Read source image from file
    let img = ImageReader::open("./tests/2.png")
        .unwrap()
        .decode()
        .unwrap();
    let _rgb = img.to_rgb8().into_vec();
    let rgba = img.to_rgba8().into_vec();
    //println!("rgb: {:?}", &rgb[..]);
    //println!("rgba: {:?}", &rgba[..]);

    let windowoptions = minifb::WindowOptions {
        borderless: false,
        transparency: false,
        title: true,
        resize: false,
        topmost: false,
        none: true,
        scale_mode: ScaleMode::Center,
        scale: Scale::X1,
    };

    let mut window = Window::new("rmg", 428, 400, windowoptions).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // Limit to max ~60 fps update rate

    // NOTE: ARGB
    let mut buffer = Vec::new();
    for f in (0..rgba.len()).step_by(4) {
        buffer.push(argb_to_u32(&rgba[f..f + 4].try_into().unwrap()));
    }

    loop {
        window
            .update_with_buffer(buffer.as_slice(), 428, 400)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}

pub fn argb_to_u32(rgba: &[u8; 4]) -> u32 {
    let a = (rgba[3] as u32) << 8 * 3;
    let r = (rgba[0] as u32) << 8 * 2;
    let g = (rgba[1] as u32) << 8 * 1;
    let b = (rgba[2] as u32) << 8 * 0;

    r + g + b + a
}
