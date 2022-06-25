use rmg::color;

fn main() {
    let rgb = [255, 255, 255];

    println!("{}", color::rgb::TransRgb::rgb_to_gray(&rgb));
}
