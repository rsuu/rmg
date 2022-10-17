



use image::io::Reader as ImageReader;


fn main() {
    // Read source image from file
    let img = ImageReader::open("./tests/1.png")
        .unwrap()
        .decode()
        .unwrap();
    let rgb = img.to_rgb8().into_raw();
    let rgba = img.to_rgba8().into_raw();
    println!("rgb: {:?}", &rgb[..]);
    println!("rgba: {:?}", &rgba[..]);
}
