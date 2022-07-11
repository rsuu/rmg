use std::{
    fs,
    path::{Path},
};

pub fn rename<T>(is_try: bool, pad: usize, names: &[T]) -> Option<Vec<String>>
where
    T: AsRef<str>,
{
    let new = pad_names(pad, names);

    if is_try {
        eprintln!("{:#?}", new);
    } else {
        // eprintln!("rename");

        for n in 0..names.len() {
            fs::rename(names[n].as_ref(), new[n].as_str()).unwrap();
        }

        return Some(new);
    }

    None
}

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
    let mut new_names = Vec::with_capacity(names.as_ref().len());

    for f in names.iter() {
        let full = Path::new(f.as_ref());

        let mut path = full.parent().unwrap().to_str().unwrap().to_string();
        let suffix = full.extension().unwrap().to_str().unwrap();
        let filename = full.file_stem().unwrap().to_str().unwrap();

        path.push('/');

        if filename.len() < pad {
            //eprintln!("{}", filename.len());

            for _ in 0..pad - filename.len() {
                path.push('0');
            }
        } else {
        }

        path.push_str(format!("{}.{}", filename, suffix).as_ref());
        new_names.push(path);
    }

    new_names
}
