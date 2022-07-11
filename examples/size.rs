use rmg::img::size::TMetaSize;
fn main() {
    let mut meta = rmg::img::size::MetaSize::<u32>::new(1440, 900, 900, 900, 1080, 1500);
    meta.resize();
    println!("{:?}", meta);

    println!(
        "{:?}",
        u64::from_be_bytes(".rmgdata".as_bytes().try_into().unwrap())
    );
}
