use infer;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_filetype(path: &Path) -> String {
    infer::get_from_path(path)
        .expect("file read successfully")
        .expect("file type is known")
        .extension()
        .to_string()
}

pub fn get_file_list(tmp_dir: &Path) -> Vec<PathBuf> {
    let mut list_file = get_dir_list(tmp_dir);

    list_file.sort_by_key(|name| name.to_lowercase()); // sort

    let mut file = PathBuf::new();
    let mut file_iter: Vec<PathBuf> = Vec::default();

    for f in list_file.into_iter() {
        file.push(tmp_dir);
        file.push(f);
        file_iter.push(file.to_path_buf());
    }

    file_iter
}

pub fn get_dir_list(path: &Path) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|f| f.ok().map(|f| f.path().display().to_string()))
        .collect::<Vec<String>>()
}
