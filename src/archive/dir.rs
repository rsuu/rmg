use crate::archive::*;
use std::{fs::OpenOptions, io::Read, path::Path};
use walkdir::WalkDir;

pub fn get_file(path: &Path, index: usize) -> anyhow::Result<Vec<u8>> {
    for (pos, tmp) in WalkDir::new(path).into_iter().enumerate() {
        let file = tmp?;

        if file.file_type().is_file() && pos == index {
            let mut file = OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(file.path())?;

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            //tracing::debug!("{}, {}", pos, index);

            return Ok(buffer);
        }
    }

    Err(anyhow::anyhow!(""))
}

pub fn get_list(path: &Path) -> anyhow::Result<FileList> {
    let mut res = FileList::new();

    for (index, tmp) in WalkDir::new(path).into_iter().enumerate() {
        let file = tmp?;

        if file.file_type().is_file() {
            res.push(FileInfo::new(
                file.path().to_str().unwrap().to_string(),
                index,
            ));
        } else {
        }
    }

    Ok(res)
}
