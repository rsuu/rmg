use rmg::files::file;
use std::path::{Path, PathBuf};

fn main() {
    let names = [
        "/tmp/.tmpNLv799/壳少/1.jpeg",
        "/tmp/.tmpNLv799/aaa/壳少/02.jpeg",
    ];
    let new = file::rename(true, 3, names.as_ref());
    println!("{:#?}", new);
}
