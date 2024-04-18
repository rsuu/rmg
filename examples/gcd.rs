use gcd;

fn main() {
    let (w, h) = (1920, 1080);

    dbg!(gcd::gcd_binary(w, h));
}
