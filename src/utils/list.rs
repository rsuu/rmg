use infer;
use std::{
    path::{Path},
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
