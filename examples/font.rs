use std::fs::File;
use std::io::Write;

fn main() {
    save_to_img();
}

fn save_to_img() {
    let font = include_bytes!("../tests/files/test.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
    let (metrics, bitmap) = font.rasterize('g', 60.0);

    let mut o = File::create("rgb.ppm").unwrap();
    o.write(format!("P6\n{} {}\n255\n", metrics.width, metrics.height).as_bytes());
    o.write(&bitmap);
}
