use rmg::archive::tar;

fn main() {
    let path = std::path::Path::new("/root/t/0.tar");
    let k = tar::get_file_list(path).unwrap();
    println!("{:?}", k);

    //   tar::ex_files2(path, k.len(), 2, 6).unwrap();
}
