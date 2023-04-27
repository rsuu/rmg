use crate::archive::*;
use std::{fs::OpenOptions, io::Read, path::Path};
use walkdir::WalkDir;

pub fn get_file<P>(path: &P, index: usize) -> anyhow::Result<Vec<u8>>
where
    P: AsRef<Path> + ?Sized,
{
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

            tracing::debug!(
                "get_file():
  pos = {}
  index = {}",
                pos,
                index
            );

            return Ok(buffer);
        }
    }

    anyhow::bail!("")
}

pub fn get_list<P>(path: &P) -> anyhow::Result<FileList>
where
    P: AsRef<Path> + ?Sized,
{
    let mut res = FileList::new();

    for (index, tmp) in WalkDir::new(path).into_iter().enumerate() {
        let file = tmp?;

        if file.file_type().is_file() {
            res.push(FileInfo::new(
                file.path().to_str().unwrap().to_string(),
                index,
            ));
        }
    }

    Ok(res)
}
