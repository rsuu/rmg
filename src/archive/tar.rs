use std::{fs::File, path::Path};
use tar::Archive;

pub fn extract(tar_path: &Path, tmp_dir: &Path) -> Result<(), std::io::Error> {
    let file = File::open(tar_path)?;
    let mut tar = Archive::new(file);
    tar.unpack(tmp_dir)?;

    Ok(())
}
