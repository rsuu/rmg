use log::debug;
use std::io::{prelude, Read, Seek};
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

pub fn load_file(tar_file: &Path, idx: usize) -> Option<Vec<u8>> {
    let file = OpenOptions::new()
        .write(false)
        .read(true)
        .create(false)
        .open(tar_file)
        .unwrap();
    let mut tar_file = tar::Archive::new(&file);
    let mut buffer = Vec::new();

    for (n, file) in tar_file.entries().unwrap().enumerate() {
        if n == idx {
            let mut f = file.unwrap();
            f.read_to_end(&mut buffer).expect("");

            return Some(buffer);
        } else {
        }
    }

    None
}

#[allow(unused)]
pub fn extract(tar_path: &Path, tmp_dir: &Path) -> Result<(), std::io::Error> {
    let file = File::open(tar_path)?;
    let mut tar = tar::Archive::new(file);
    tar.unpack(tmp_dir)?;

    Ok(())
}

pub fn get_file_list(path: &Path) -> Result<Vec<(String, usize)>, std::io::Error> {
    let file = OpenOptions::new()
        .write(false)
        .read(true)
        .create(false)
        .open(path)?;
    let mut tar = tar::Archive::new(file);
    let mut list = Vec::new();

    for (idx, f) in tar.entries()?.enumerate() {
        list.push((f?.path()?.display().to_string(), idx));
    }

    Ok(list)
}
