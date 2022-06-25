use rmg::metadata::meta::MetaData;

fn main() {
    let mut meta = MetaData::new();
    meta.temp = Some("adwad".to_string());

    meta.write_to_file("./w.bin");
    meta.read_from_file("./w.bin");

    println!("{:?}", meta);
}
