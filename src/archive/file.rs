use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

use super::utils::{FileInfo, FileList};

pub fn get_file(path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(path)?;

    let mut res = Vec::new();
    file.read_to_end(&mut res)?;

    Ok(res)
}

pub fn get_list(path: &Path) -> anyhow::Result<FileList> {
    let mut res = FileList::new();
    res.push(FileInfo::new(path.display().to_string(), 0));

    Ok(res)
}
