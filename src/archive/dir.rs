use crate::utils::err::{MyErr, Res};

use std::{fs::OpenOptions, io::Read, path::Path};
use walkdir;

pub fn load_file(path: &Path, pos: usize) -> Res<Vec<u8>> {
    for (idx, f) in walkdir::WalkDir::new(path).into_iter().enumerate() {
        if pos == idx {
            let mut buffer = Vec::new();
            let mut f = OpenOptions::new()
                .write(false)
                .read(true)
                .create(false)
                .open(f.unwrap().path())
                .unwrap();

            f.read_to_end(&mut buffer).unwrap();

            return Ok(buffer);
        } else {
        }
    }

    Err(MyErr::Null(()))
}

pub fn get_file_list(path: &Path) -> Res<Vec<(String, usize)>> {
    let mut list = Vec::new();

    for (idx, f) in walkdir::WalkDir::new(path).into_iter().enumerate() {
        list.push((f?.path().to_str().unwrap().to_string(), idx));
    }

    Ok(list)
}
