use crate::{
    archive::{self, ArchiveType},
    img::size::TMetaSize,
    render::view::{ImgFormat, Page},
    utils::err::{MyErr, Res},
    EXT_LIST,
};
use infer;
use std::path::Path;

pub fn get_path_type(path: impl AsRef<Path>) -> ArchiveType {
    if path.as_ref().is_dir() {
        return ArchiveType::Dir;
    }

    let Ok(tmp) = infer::get_from_path(path.as_ref()) else {
        panic!()
    };

    match tmp {
        Some(t) => match t.extension() {
            "tar" => ArchiveType::Tar,
            "zip" => ArchiveType::Zip,
            _ => {
                todo!()
            }
        },

        None => ArchiveType::File,
    }
}

pub fn get_img_type(path: impl AsRef<Path>) -> ImgFormat {
    let ty = infer::get_from_path(path.as_ref()).unwrap();

    match ty {
        Some(v) => ImgFormat::from(v.extension().to_string().as_str()),
        None => ImgFormat::from(path.as_ref().extension().unwrap().to_str().unwrap()),
    }
}

// TODO: rewrite
pub fn has_supported(path: impl AsRef<Path>) -> bool {
    if path.as_ref().is_dir() {
        return false;
    }

    for ext in EXT_LIST {
        // HINT: non-UTF-8 path is allow
        if path
            .as_ref()
            .display()
            .to_string()
            .as_str()
            .ends_with(format!(".{}", ext).as_str())
        {
            return true;
        } else {
            // skip
        }
    }

    false
}

// pad = 6
//     '01.jpg'        -> '000001.jpg'     (push  "0000" )
//     '000001.jpg'    -> '000001.jpg'     (doing nothing)
//     '000000001.jpg' -> '0000000001.jpg' (doing nothing)
pub fn pad_name(pad: usize, name: &str) -> String {
    let full = Path::new(name);

    let mut path = full.parent().unwrap().to_str().unwrap().to_string();
    let suffix = full.extension().unwrap().to_str().unwrap();
    let filename = full.file_stem().unwrap().to_str().unwrap();

    path.push('/');

    if filename.len() < pad {
        for _ in 0..pad - filename.len() {
            path.push('0');
        }
    }

    path.push_str(format!("{}.{}", filename, suffix).as_ref());

    log::debug!("path = {:?}", path);

    path
}

#[inline(always)]
pub fn is_same_slice(foo: &[u8], bar: &[u8], start: usize, len: usize) -> Res<bool> {
    if foo.len() > start + len && &foo[start..start + len] == bar {
        Ok(true)
    } else {
        Err(MyErr::Null)
    }
}

pub fn is_aseprite(bytes: &[u8]) -> bool {
    is_same_slice(bytes, &[0xe0, 0xa5], 4, 2).is_ok()
}

pub fn get_page_list(file_list: &[(String, usize)], rename_pad: usize) -> Vec<Page> {
    let mut page_list = Vec::new();

    for (path, pos) in file_list.iter() {
        if has_supported(AsRef::<Path>::as_ref(path.as_str())) {
            let page = if rename_pad == 0 {
                Page::new(path.clone(), *pos)
            } else {
                // rename
                Page::new(pad_name(rename_pad, path.as_str()), *pos)
            };

            page_list.push(page);
        } else {
            // skip
        }
    }

    page_list
}

pub fn get_file_list(path: impl AsRef<Path>) -> Res<Vec<(String, usize)>> {
    let ty = get_path_type(path.as_ref());

    match ty {
        ArchiveType::Dir => archive::dir::get_file_list(path.as_ref()),

        ArchiveType::File => {
            // e.g. rmg test.gif
            let ty = get_img_type(path.as_ref());

            if ty == ImgFormat::Unknown {
                // unsupport
                return Err(MyErr::Todo);
            }

            // Only one file.
            Ok(vec![(path.as_ref().display().to_string(), 0)])
        }

        ArchiveType::Tar => archive::tar::get_file_list(path.as_ref()),

        ArchiveType::Zip => archive::zip::get_file_list(path.as_ref()),
    }
}
