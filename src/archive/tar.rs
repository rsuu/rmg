use crate::{archive::*, Path};

pub fn get_file<P>(path: &P, index: usize) -> eyre::Result<Vec<u8>>
where
    P: AsRef<Path> + ?Sized,
{
    #[cfg(feature = "ex_tar")]
    {
        return feat::get_file(path, index);
    }

    eyre::bail!("")
}

pub fn get_list<P>(path: &P) -> eyre::Result<FileList>
where
    P: AsRef<Path> + ?Sized,
{
    #[cfg(feature = "ex_tar")]
    {
        return feat::get_list(path);
    }

    eyre::bail!("")
}

#[cfg(feature = "ex_tar")]
mod feat {
    use crate::archive::*;
    use std::{fs::OpenOptions, io::Read, path::Path};
    extern crate tar;

    pub fn get_list<P>(path: &P) -> eyre::Result<FileList>
    where
        P: AsRef<Path> + ?Sized,
    {
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

    pub fn get_file<P>(tar_file: &P, index: usize) -> eyre::Result<Vec<u8>>
    where
        P: AsRef<Path> + ?Sized,
    {
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
            }
        }

        eyre::bail!("")
    }
}
