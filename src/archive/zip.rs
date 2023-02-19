use crate::archive::*;
use std::path::Path;

pub fn get_file<_Path>(path: &_Path, index: usize) -> anyhow::Result<Vec<u8>>
where
    _Path: AsRef<Path> + ?Sized,
{
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_zip")] {
            feat::get_file(path.as_ref(), index)
        } else {
            Err(anyhow::anyhow!(""))
        }
    }
}

pub fn get_list<_Path>(path: &_Path) -> anyhow::Result<FileList>
where
    _Path: AsRef<Path> + ?Sized,
{
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_zip")] {
            feat::get_list(path.as_ref())
        } else {
            Err(anyhow::anyhow!(""))
        }
    }
}

pub fn extract<_Path>(from: &_Path, to: &_Path) -> anyhow::Result<()>
where
    _Path: AsRef<Path> + ?Sized,
{
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_zip")] {
            feat::extract(from.as_ref(), to.as_ref())
        } else {
            Err(anyhow::anyhow!(""))
        }
    }
}

#[cfg(feature = "ex_zip")]
mod feat {
    use crate::archive::*;
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
    };
    extern crate zip;

    #[inline]
    pub fn extract<_Path>(zip_path: &_Path, target: &_Path) -> anyhow::Result<()>
    where
        _Path: AsRef<Path> + ?Sized,
    {
        let mut zip = zip::ZipArchive::new(File::open(zip_path.as_ref())?).unwrap();

        zip.extract(target.as_ref()).unwrap();

        Ok(())
    }

    #[inline]
    pub fn get_list<_Path>(path: &_Path) -> anyhow::Result<FileList>
    where
        _Path: AsRef<Path> + ?Sized,
    {
        let mut res = FileList::new();

        let file = File::open(path.as_ref()).unwrap();
        let render = BufReader::new(file);
        let mut zip = zip::ZipArchive::new(render).unwrap();

        for index in 0..zip.len() {
            let file = zip.by_index(index).unwrap();

            res.push(FileInfo::new(file.name().to_string(), index));
        }

        Ok(res)
    }

    #[inline]
    pub fn get_file<_Path>(path: &_Path, index: usize) -> anyhow::Result<Vec<u8>>
    where
        _Path: AsRef<Path> + ?Sized,
    {
        let file = File::open(path.as_ref()).unwrap();
        let render = BufReader::new(file);
        let mut zip = zip::ZipArchive::new(render).unwrap();
        let mut file = zip.by_index(index).unwrap();

        let mut res = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut res).unwrap();

        Ok(res)
    }
}
