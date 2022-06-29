use rmg::{
    metadata::{
        meta::MetaData,
        tags::{self, TransTag},
    },
    utils::types::Names,
};

fn main() {
    let mut meta = MetaData::new();
    meta.temp = Some("adwad".to_string());
    meta.male = Some("adwad".to_string());

    meta.write_to_file("./w.rmg").unwrap();
    meta.read_from_file("./w.rmg").unwrap();

    let n: Names = vec![
        tags::TagReclass::Doujinshi.to_string(),
        tags::TagReclass::Misc.to_string(),
    ];
    meta.reclass = Some(n.to_string());
    println!("{:?}", meta);
}
