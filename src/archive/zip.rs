use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, path::Path};
use zip::ZipArchive;

pub fn extract<_Path>(zip_path: &_Path, target: &_Path) -> Result<(), std::io::Error>
where
    _Path: AsRef<Path> + ?Sized,
{
    let mut zip = ZipArchive::new(File::open(zip_path.as_ref())?)?;

    zip.extract(target.as_ref()).unwrap();

    Ok(())
}

pub fn get_file_list<_Path>(path: &_Path) -> Result<Vec<(String, usize)>, ()>
where
    _Path: AsRef<Path> + ?Sized,
{
    let mut res: Vec<(String, usize)> = Vec::new();

    let file = File::open(path.as_ref()).unwrap();
    let reader = BufReader::new(file);
    let mut zip = zip::ZipArchive::new(reader).unwrap();

    for idx in 0..zip.len() {
        let file = zip.by_index(idx).unwrap();

        res.push((file.name().to_string(), idx));
    }

    Ok(res)
}

pub fn load_file<_Path>(path: &_Path, idx: usize) -> Result<Vec<u8>, ()>
where
    _Path: AsRef<Path> + ?Sized,
{
    let file = File::open(path.as_ref()).unwrap();
    let reader = BufReader::new(file);
    let mut zip = zip::ZipArchive::new(reader).unwrap();
    let mut file = zip.by_index(idx).unwrap();

    let mut res = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut res).unwrap();

    Ok(res)
}
