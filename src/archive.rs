pub mod dir;
pub mod file;

// feature
pub mod tar;
pub mod zip;

// ==============================================
use crate::{
    archive,
    img::TMetaSize,
    render::{ImgFormat, Page},
    EXT_LIST,
};
use infer;
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone)]
pub enum ArchiveType {
    Tar,
    Zip,
    Dir,
    File,
}

#[derive(Debug)]
pub struct FileList {
    inner: Vec<FileInfo>,
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: String,
    pub index: usize,
    //    pub fmt: ImgFormat,
    //    pub ty: ImgType,
    //    pub size: Size<u32>,
    //    pub resize: Size<u32>,
}

pub trait ForExtract {
    fn get_list(&self, path: &Path) -> anyhow::Result<FileList>;
    fn get_file(&self, path: &Path, index: usize) -> anyhow::Result<Vec<u8>>;
}

impl ForExtract for ArchiveType {
    fn get_list(&self, path: &Path) -> anyhow::Result<FileList> {
        match *self {
            Self::File => archive::file::get_list(path),
            Self::Dir => archive::dir::get_list(path),

            Self::Tar => archive::tar::get_list(path),
            Self::Zip => archive::zip::get_list(path),
        }
    }

    fn get_file(&self, path: &Path, index: usize) -> anyhow::Result<Vec<u8>> {
        match *self {
            Self::File => archive::file::get_file(path),
            Self::Dir => archive::dir::get_file(path, index),

            Self::Tar => archive::tar::get_file(path, index),
            Self::Zip => archive::zip::get_file(path, index),
        }
    }
}

impl ArchiveType {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        if path.is_dir() {
            return Ok(ArchiveType::Dir);
        }

        let Ok(ty) = infer::get_from_path(path) else {
            // BUG: has space in the path
            anyhow::bail!("Unknown Type")
        };

        let res = match ty {
            Some(ext) => match ext.extension() {
                "tar" => Self::Tar,
                "zip" => Self::Zip,
                // "rar"=>Self::Rar,
                // "7z"=>Self::7z,
                _ => Self::File,
            },

            None => Self::File,
        };

        Ok(res)
    }
}

impl FileList {
    pub fn to_page_list(&self, rename_pad: usize) -> Vec<Page> {
        let mut res = Vec::new();

        for info in self.iter() {
            if info.has_supported() {
                let page = {
                    if rename_pad == 0 {
                        Page::new(info.path.clone(), info.index)
                    } else {
                        // rename
                        Page::new(pad_name(rename_pad, info.path.as_str()), info.index)
                    }
                };

                res.push(page);
            }
        }

        res
    }
}

impl FileList {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn push(&mut self, value: FileInfo) {
        self.inner.push(value);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, FileInfo> {
        self.inner.iter()
    }
}

impl FileInfo {
    pub fn new(path: String, index: usize) -> Self {
        Self { path, index }
    }

    // TODO: rewrite
    pub fn has_supported(&self) -> bool {
        let path = PathBuf::from(self.path.as_str());

        if path.is_dir() {
            return false;
        }

        for ext in EXT_LIST {
            // WARN: allow non-UTF-8 path
            if path
                .display()
                .to_string()
                .ends_with(format!(".{ext}").as_str())
            {
                return true;
            }
        }

        false
    }
}

// e.g. width = 6
//     '01.jpg'        -> '000001.jpg'     (pad  "0000")
//     '000001.jpg'    -> '000001.jpg'     (do nothing)
//     '000000001.jpg' -> '0000000001.jpg' (do nothing)
pub fn pad_name(width: usize, name: &str) -> String {
    let full = Path::new(name);

    let mut path = full.parent().unwrap().to_str().unwrap().to_string();
    let suffix = full.extension().unwrap().to_str().unwrap();
    let filename = full.file_stem().unwrap().to_str().unwrap();

    path.push('/');

    if filename.len() < width {
        path.extend(vec!['0'; (width - filename.len())]);
    }

    path.push_str(format!("{filename}.{suffix}").as_ref());

    //tracing::debug!("path = {:?}", path);

    path
}

pub fn is_same_slice(foo: &[u8], bar: &[u8], start: usize, len: usize) -> anyhow::Result<bool> {
    if foo.len() > start + len && &foo[start..start + len] == bar {
        Ok(true)
    } else {
        anyhow::bail!("")
    }
}

pub fn get_img_format(file: &[u8]) -> ImgFormat {
    let ty = infer::get(file);

    match ty {
        Some(v) => ImgFormat::from(v.extension().to_string().as_str()),
        None => {
            if is_same_slice(file, &[0xe0, 0xa5], 4, 2).is_ok() {
                ImgFormat::Aseprite
            } else {
                panic!("Unknown Format")
            }
        }
    }
}
