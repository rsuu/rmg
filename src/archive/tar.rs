
use std::{fs::File, path::Path};
use tar::Archive;

pub fn extract(tar_path: &Path, tmp_dir: &Path) -> Result<(), std::io::Error> {
    let file = File::open(tar_path)?;
    let mut tar = Archive::new(file);
    tar.unpack(tmp_dir)?;

    Ok(())
}

pub fn get_file_list(tar_path: &Path) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(tar_path)?;
    let mut tar = Archive::new(file);
    let mut list = Vec::new();

    for f in tar.entries()? {
        list.push(f?.path()?.display().to_string());
    }

    Ok(list)
}

// pub fn ex_files(
//     tar_path: &Path,
//     files_len: usize,
//     start: usize,
//     len: usize,
// ) -> Result<bool, std::io::Error> {
//     let file = File::open(tar_path)?;
//     let mut tar = Archive::new(file);
//
//     let end = start + len;
//
//     if end <= files_len {
//         tar.entries()?.enumerate().for_each(|(n, f)| {
//             if n >= start && n <= end {
//                 let mut f = f.expect("");
//                 let filename = f.path().expect("");
//
//                 let mut out = std::fs::File::create(filename.as_ref()).unwrap();
//                 copy(&mut f, &mut out).expect("");
//             }
//         });
//     } else {
//         // This way is Not good
//         return Ok(false);
//     }
//
//     Ok(true)
// }
//
// pub fn ex_files2(
//     tar_path: &Path,
//     files_len: usize,
//     start: usize,
//     len: usize,
// ) -> Result<bool, std::io::Error> {
//     let file = File::open(tar_path)?;
//     let mut tar = Archive::new(file);
//
//     let end = if start + len > files_len {
//         files_len
//     } else {
//         start + len
//     };
//
//     tar.entries()?.enumerate().for_each(|(n, f)| {
//         if n >= start && n <= end {
//             let mut f = f.expect("");
//             let filename = f.path().expect("");
//
//             let mut out = std::fs::File::create(filename.as_ref()).unwrap();
//             copy(&mut f, &mut out).expect("");
//         }
//     });
//
//     Ok(true)
// }
