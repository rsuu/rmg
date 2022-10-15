use std::{path::Path};

/// ```text
/// if pad == 6
/// '01.jpg' -> '000001.jpg'            (push "0000" at head)
/// '000001.jpg' -> '000001.jpg'        (doing nothing)
/// '000000001.jpg' -> '0000000001.jpg' (doing nothing)
/// ```
pub fn pad_names<T>(pad: usize, names: &[T]) -> Vec<String>
where
    T: AsRef<str>,
{
    let mut res = Vec::with_capacity(names.as_ref().len());

    for f in names.iter() {
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
    log::debug!("{:?}", path);

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
