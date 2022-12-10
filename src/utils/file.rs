use infer;
use std::path::Path;

use super::err::Res;

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

/// ```text
/// if pad == 6
/// '01.jpg'        -> '000001.jpg'     (push  "0000" )
/// '000001.jpg'    -> '000001.jpg'     (doing nothing)
/// '000000001.jpg' -> '0000000001.jpg' (doing nothing)
/// ```
pub fn pad_names<T>(pad: usize, list: &[T]) -> Vec<String>
where
    T: AsRef<str>,
{
    let mut res = Vec::with_capacity(list.as_ref().len());

    for f in list.iter() {
        let full = Path::new(f.as_ref());

        let mut path = full.parent().unwrap().to_str().unwrap().to_string();
        let suffix = full.extension().unwrap().to_str().unwrap();
        let filename = full.file_stem().unwrap().to_str().unwrap();

        path.push('/');
        log::debug!("{:?}", path);

        if filename.len() < pad {
            //eprintln!("{}", filename.len());

            for _ in 0..pad - filename.len() {
                path.push('0');
            }
        } else {
        }

        path.push_str(format!("{}.{}", filename, suffix).as_ref());
        res.push(path);
    }

    res
}

pub fn pad_name(pad: usize, name: &str) -> String {
    let full = Path::new(name);

    let mut path = full.parent().unwrap().to_str().unwrap().to_string();
    let suffix = full.extension().unwrap().to_str().unwrap();
    let filename = full.file_stem().unwrap().to_str().unwrap();

    path.push('/');

    log::debug!("$path = {:?}", path);

    if filename.len() < pad {
        //eprintln!("{}", filename.len());

        for _ in 0..pad - filename.len() {
            path.push('0');
        }
    } else {
    }

    path.push_str(format!("{}.{}", filename, suffix).as_ref());

    path
}

#[inline(always)]
pub fn is_same_slice(foo: &[u8], bar: &[u8], start: usize, len: usize) -> Res<bool> {
    if foo.len() > start + len && &foo[start..start + len] == bar {
        Ok(true)
    } else {
        Err(crate::utils::err::MyErr::Null)
    }
}

pub fn is_aseprite(bytes: &[u8]) -> bool {
    is_same_slice(bytes, &[0xe0, 0xa5], 4, 2).is_ok()
}
