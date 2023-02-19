use crate::archive::*;
use std::path::Path;

pub fn get_file(path: &Path, index: usize) -> anyhow::Result<Vec<u8>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_tar")] {
            feat::get_file(path, index)
        } else {
            Err(anyhow::anyhow!(""))
        }
    }
}

pub fn get_list(path: &Path) -> anyhow::Result<FileList> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_tar")] {
            feat::get_list(path)
        } else {
            Err( anyhow::anyhow!(""))
        }
    }
}

#[cfg(feature = "ex_tar")]
mod feat {
    use crate::archive::*;
    use std::{fs::OpenOptions, io::Read, path::Path};
    extern crate tar;

    #[inline]
    pub fn get_list(path: &Path) -> anyhow::Result<FileList> {
        let mut tar = {
            let file = OpenOptions::new()
                .read(true)
                .write(false)
                .create(false)
                .open(path)?;

            tar::Archive::new(file)
        };

        let mut res = FileList::new();

        for (index, file) in tar.entries()?.enumerate() {
            let path = file?.path()?.clone().to_str().unwrap().to_string();

            if path.ends_with('/') {
            } else {
                res.push(FileInfo::new(path, index));
            }
        }

        Ok(res)
    }

    #[inline]
    pub fn get_file(tar_file: &Path, index: usize) -> anyhow::Result<Vec<u8>> {
        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(tar_file)?;
        let mut tar_file = tar::Archive::new(&file);
        let mut buffer = vec![];

        for (n, file) in tar_file.entries()?.enumerate() {
            if n == index {
                file?.read_to_end(&mut buffer).expect("");

                return Ok(buffer);
            } else {
            }
        }

        Err(anyhow::anyhow!(""))
    }
}
