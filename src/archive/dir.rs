use crate::utils::err::{MyErr, Res};
use std::{fs::OpenOptions, io::Read, path::Path};
use walkdir::WalkDir;

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
    let mut idx = 0;

    for tmp in WalkDir::new(path.as_ref()).into_iter() {
        if tmp.as_ref().unwrap().file_type().is_file() {
            if pos == idx {
                let mut buffer = Vec::new();
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(tmp?.path())?;

                file.read_to_end(&mut buffer)?;

                tracing::debug!("{}, {}", idx, pos);

                return Ok(buffer);
            } else {
                idx += 1;
            }
        } else {
        }
    }

    Err(MyErr::Null)
}

pub fn get_file_list(path: impl AsRef<Path>) -> Res<Vec<(String, usize)>> {
    let mut list = Vec::new();
    let mut idx = 0;

    for file in WalkDir::new(path.as_ref()).into_iter() {
        if file.as_ref().unwrap().file_type().is_file() {
            list.push((file?.path().to_str().unwrap().to_string(), idx));
            idx += 1;
        } else {
        }
    }

    Ok(list)
}
