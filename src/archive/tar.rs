extern crate tar;

use log::debug;
use std::io::{prelude, Read, Seek, Write};
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

pub fn load_file(tar_file: &Path, name_path: &Path) -> Option<Vec<u8>> {
    let file = OpenOptions::new()
        .write(false)
        .read(true)
        .create(false)
        .open(tar_file)
        .unwrap();
    let mut tar_file = tar::Archive::new(&file);
    let mut buffer = Vec::new();

    for file in tar_file.entries().unwrap() {
        let mut f = file.unwrap();

        if f.header().path().as_deref().unwrap() == name_path {
            f.read_to_end(&mut buffer).expect("");

            return Some(buffer);
        } else {
        }
    }

    None
}

pub fn extract(tar_path: &Path, tmp_dir: &Path) -> Result<(), std::io::Error> {
    let file = File::open(tar_path)?;
    let mut tar = tar::Archive::new(file);
    tar.unpack(tmp_dir)?;

    Ok(())
}

pub fn get_file_list(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let file = OpenOptions::new()
        .write(false)
        .read(true)
        .create(false)
        .open(path)?;
    let mut tar = tar::Archive::new(file);
    let mut list = Vec::new();

    for f in tar.entries()? {
        list.push(f?.path()?.display().to_string());
    }

    Ok(list)
}
