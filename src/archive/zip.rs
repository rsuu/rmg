use crate::*;

use ::zip::ZipArchive;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn get_file<P>(path: &P, index: usize) -> eyre::Result<Vec<u8>>
where
    P: AsRef<Path> + ?Sized,
{
    let file = File::open(path.as_ref())?;
    let render = BufReader::new(file);
    let mut zip = ZipArchive::new(render)?;
    let mut file = zip.by_index(index)?;

    let mut res = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut res)?;

    Ok(res)
}

pub fn get_list<P>(path: &P) -> eyre::Result<FileList>
where
    P: AsRef<Path> + ?Sized,
{
    let mut res = FileList::new();

    let file = File::open(path.as_ref())?;
    let render = BufReader::new(file);
    let mut zip = ZipArchive::new(render)?;

    for index in 0..zip.len() {
        let file = zip.by_index(index)?;

        res.push(FileInfo::new(file.name().to_string(), index));
    }

    Ok(res)
}

pub fn extract<P>(src: &P, dst: &P) -> eyre::Result<()>
where
    P: AsRef<Path> + ?Sized,
{
    let mut zip = ZipArchive::new(File::open(src.as_ref())?)?;

    zip.extract(dst.as_ref())?;

    Ok(())
}
