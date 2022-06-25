fn main() {
    let mut v = Vec::new();
    let a = [1_u8; 1024000];

    for _f in 1..10 {
        v.clear();
        v.extend_from_slice(a.as_slice());
    }
}
