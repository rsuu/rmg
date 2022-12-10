use crate::utils::err::{MyErr, Res};
use std::{fs::OpenOptions, io::Read, path::Path};
use walkdir;

pub fn load_file(path: impl AsRef<Path>) -> Res<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut file = OpenOptions::new()
        .write(false)
        .read(true)
        .create(false)
        .open(path.as_ref())?;

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn load_dir(path: impl AsRef<Path>, pos: usize) -> Res<Vec<u8>> {
    for (idx, tmp) in walkdir::WalkDir::new(path.as_ref()).into_iter().enumerate() {
        if pos == idx {
            let mut buffer = Vec::new();
            let mut file = OpenOptions::new()
                .write(false)
                .read(true)
                .create(false)
                .open(tmp?.path())?;

            file.read_to_end(&mut buffer)?;

            // done
            return Ok(buffer);
        } else {
            // to next
        }
    }

    Err(MyErr::Null)
}

pub fn get_file_list(path: impl AsRef<Path>) -> Res<Vec<(String, usize)>> {
    let mut list = Vec::new();

    for (idx, file) in walkdir::WalkDir::new(path.as_ref()).into_iter().enumerate() {
        list.push((file?.path().to_str().unwrap().to_string(), idx));
    }

    Ok(list)
}
