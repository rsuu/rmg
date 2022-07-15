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
