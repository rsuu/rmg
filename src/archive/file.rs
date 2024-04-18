use crate::*;
use std::{fs::OpenOptions, io::Read, path::Path};

pub fn get_file<P>(path: &P) -> eyre::Result<Vec<u8>>
where
    P: AsRef<Path> + ?Sized,
{
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(path)?;

    let mut res = Vec::new();
    file.read_to_end(&mut res)?;

    Ok(res)
}

pub fn get_list(path: &Path) -> eyre::Result<FileList> {
    let mut res = FileList::new();
    res.push(FileInfo::new(path.display().to_string(), 0));

    Ok(res)
}
