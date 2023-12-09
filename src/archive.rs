// TODO: [7z, rar, zstd]

pub mod dir;
pub mod file;

// feature
pub mod tar;
pub mod zip;

// ==============================================
use crate::{archive, ImgFormat, Page, TMetaSize, EXT_LIST};
use infer;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileList {
    inner: Vec<FileInfo>,
    // TODO: ?anymore
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: String,
    pub index: usize,
    // TODO: ?
    // pub fmt: ImgFormat,
    // pub ty: ImgType,
    // pub size: Size<u32>,
    // pub resize: Size<u32>,
}

#[derive(Debug, Copy, Clone)]
pub enum ArchiveType {
    Tar,
    Zip,
    Dir,
    File,
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

        // BUG: remove space in the path
        let Ok(ty) = infer::get_from_path(path) else {
            anyhow::bail!("Unknown Type")
        };

        let Some(ext) = ty else {
            // Not archive.
            return Ok(Self::File);
        };

        let res = match ext.extension() {
            "tar" => Self::Tar,
            "zip" => Self::Zip,
            // "rar"=>Self::Rar,
            // "7z"=>Self::7z,
            // "tzst" | "zst"=>Self::Zstd,
            _ => Self::File,
        };

        Ok(res)
    }
}

impl FileList {
    pub fn to_page_list(&self, rename_pad: usize) -> Vec<Page> {
        let mut res = Vec::with_capacity(self.len());

        for info in self.iter() {
            if !info.is_supported() {
                continue;
            }

            let path = {
                if rename_pad == 0 {
                    // as-is
                    info.path.clone()
                } else {
                    // rename
                    pad_name(rename_pad, info.path.as_str())
                }
            };
            let page = Page::new(path, info.index);

            res.push(page);
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

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl FileInfo {
    pub fn new(path: String, index: usize) -> Self {
        Self { path, index }
    }

    pub fn is_supported(&self) -> bool {
        let path = PathBuf::from(self.path.as_str());

        // if path.is_dir() {
        //     return false;
        // }

        let Some(val) = path.extension() else {
            return false;
        };
        let val = val.to_str().unwrap();

        EXT_LIST.contains(&val)
    }
}

// e.g. width = 6
//     01.jpg        -> 000001.jpg     (rename)
//     000001.jpg    -> 000001.jpg     (as-is)
//     000000001.jpg -> 0000000001.jpg (as-is)
pub fn pad_name(width: usize, name: &str) -> String {
    let full = Path::new(name);

    // TODO: rewrite(shit)
    let mut path = full.parent().unwrap().to_str().unwrap().to_string();
    let suffix = full.extension().unwrap().to_str().unwrap();
    let filename = full.file_stem().unwrap().to_str().unwrap();

    // padding with `0`
    let tmp = format!("{0:0>width$}", filename);
    let tmp = format!("/{tmp}.{suffix}");
    path.push_str(tmp.as_str());

    tracing::trace!(
        "
pad_name():
  path = {:?}",
        &path
    );

    path
}

pub fn is_same_slice(foo: &[u8], bar: &[u8], start: usize, len: usize) -> bool {
    (foo.len() > start + len) && (&foo[start..start + len] == bar)
}

// TODO: with crate
//       ?svg
pub fn get_img_format(file: &[u8]) -> ImgFormat {
    let ty = infer::get(file);

    if let Some(v) = ty {
        ImgFormat::from(v.extension().to_string().as_str())
    } else if is_same_slice(file, &[0xe0, 0xa5], 4, 2) {
        ImgFormat::Aseprite
    } else {
        panic!("Unknown Format")
    }
}
