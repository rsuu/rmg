use crate::archive::*;
use std::{fs::OpenOptions, io::Read, path::Path};
use walkdir::WalkDir;

pub fn get_file<P>(path: &P, index: usize) -> eyre::Result<Vec<u8>>
where
    P: AsRef<Path> + ?Sized,
{
    let mut pos = 0;
    for file in WalkDir::new(path).contents_first(true).into_iter() {
        let file = file?;

        // if !file.file_type().is_file() {
        //     continue;
        // }

        if pos == index {
            let mut file = OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(file.path())?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            // tracing::debug!(
            //     "get_file():
            //  pos = {}
            //  index = {}",
            //     pos,
            //     index
            // );

            return Ok(buffer);
        } else {
            pos += 1;
        }
    }

    eyre::bail!(format!(
        "
ERROR: Not found file with index `{index}` in dir `{}`
",
        path.as_ref().display(),
    ))
}

pub fn get_list<P>(path: &P) -> eyre::Result<FileList>
where
    P: AsRef<Path> + ?Sized,
{
    let mut res = FileList::new();

    for (index, tmp) in WalkDir::new(path).into_iter().enumerate() {
        let file = tmp?;

        if file.file_type().is_file() {
            res.push(FileInfo::new(file.path().display().to_string(), index));
        }
    }

    Ok(res)
}
