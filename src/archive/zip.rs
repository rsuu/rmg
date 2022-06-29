use std::{fs::File, path::Path};
use zip::ZipArchive;

pub fn extract(zip_path: &Path, target: &Path) -> Result<(), std::io::Error> {
    let mut zip = ZipArchive::new(File::open(&zip_path)?)?;
    zip.extract(target)?;
    Ok(())
}
