use infer;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_filetype<T>(path: &T) -> String
where
    T: AsRef<Path> + ?Sized,
{
    infer::get_from_path(path.as_ref())
        .expect("file read successfully")
        .expect("file type is known")
        .extension()
        .to_string()
}

pub fn get_file_list<T>(tmp_dir: &T) -> Vec<PathBuf>
where
    T: AsRef<Path> + ?Sized,
{
    let list_file = get_dir_list(tmp_dir.as_ref());

    // list_file.sort_by_key(|name| name.to_lowercase()); // sort

    let mut file_path = PathBuf::new();
    let mut file_iter: Vec<PathBuf> = Vec::new();

    for f in list_file.iter() {
        file_path.push(tmp_dir);
        file_path.push(f);
        file_iter.push(file_path.clone());
        file_path.clear();
    }

    file_iter
}

pub fn get_dir_list<T>(path: &T) -> Vec<PathBuf>
where
    T: AsRef<Path> + ?Sized,
{
    fs::read_dir(path.as_ref())
        .unwrap()
        .filter_map(|f| f.ok().map(|f| f.path()))
        .collect::<Vec<PathBuf>>()
}
