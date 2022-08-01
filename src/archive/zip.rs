use std::io::prelude::*;
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

pub fn get_filelist<R>(reader: R) -> zip::result::ZipResult<()>
where
    R: Read + Seek,
{
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
        //std::io::copy(&mut file, &mut std::io::stdout());
    }

    Ok(())
}
