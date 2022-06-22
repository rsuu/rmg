use std::{fs::File, path::Path};

use zip::ZipArchive;

pub fn extract(file_path: &Path, target: &Path) -> Result<(), std::io::Error> {
    let mut zip = ZipArchive::new(File::open(&file_path)?)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.is_dir() {
        } else {
            let file_path = target.join(Path::new(file.name()));
            let mut target_file = if file_path.exists() {
                File::open(file_path)?
            } else {
                File::create(file_path)?
            };
            std::io::copy(&mut file, &mut target_file)?;
        }
    }
    Ok(())
}
