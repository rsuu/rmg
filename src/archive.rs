pub mod dir;
pub mod file;
pub mod zip;

// feature
pub mod tar;

use crate::*;

// ==============================================
use infer;

pub enum DataType {
    Archive {
        path: PathBuf,
        fmt: ArchiveFmt,
        filelist: FileList,
    },

    Dir {
        path: PathBuf,
        filelist: FileList,
    },

    SingleImg {
        path: PathBuf,
    },

    Unknown,
}

#[derive(Debug)]
pub struct FileList {
    pub inner: Vec<FileInfo>,
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: String,
    pub index: usize,
}

pub enum ArchiveFmt {
    Zip,
    Tar,
}

impl FileList {
    pub fn gen_empty_pages(&self, fname_padding: usize) -> Vec<Page> {
        let mut tmp = Vec::with_capacity(self.len());

        for info in self.iter() {
            if !info.is_supported() {
                continue;
            }

            let pad_path = {
                // as-is
                if fname_padding == 0 {
                    info.path.clone()
                } else {
                    padding_filename(fname_padding, info.path.as_str())
                }
            };

            tmp.push((info, pad_path));
        }

        // sort by pad_path
        tmp.sort_by(|a, b| a.1.as_str().partial_cmp(b.1.as_str()).unwrap());

        let mut res = vec![];
        for (index, (info, ..)) in tmp.iter().enumerate() {
            res.push(Page::new_empty(index))
        }

        res
    }
}

impl DataType {
    pub fn new(path: &Path) -> eyre::Result<Self> {
        if path.is_dir() {
            return Ok(Self::Dir {
                filelist: archive::dir::get_list(path)?,
                path: path.to_path_buf(),
            });
        }

        // FIXME: remove space in the path
        let Ok(ty) = infer::get_from_path(path) else {
            eyre::bail!("Unknown Type")
        };

        let Some(t) = ty else {
            // Not archive.
            return Ok(Self::SingleImg {
                path: path.to_path_buf(),
            });
        };
        let ext = t.extension();

        Ok(match ext {
            "zip" => Self::Archive {
                fmt: ArchiveFmt::Zip,
                filelist: archive::zip::get_list(path)?,
                path: path.to_path_buf(),
            },

            "tar" => Self::Archive {
                fmt: ArchiveFmt::Tar,
                filelist: archive::tar::get_list(path)?,
                path: path.to_path_buf(),
            },

            _ if SUPPORTED_FORMAT.contains(&ext) => Self::SingleImg {
                path: path.to_path_buf(),
            },

            _ => Self::Unknown,
        })
    }

    pub fn file_nums(&self) -> usize {
        match self {
            DataType::Archive { filelist, .. } | DataType::Dir { filelist, .. } => filelist.len(),
            DataType::SingleImg { .. } => 1,
            _ => unreachable!(),
        }
    }

    pub fn gen_empty_pages(&self, fname_padding: usize) -> eyre::Result<Vec<Page>> {
        Ok(match &self {
            Self::Archive { filelist, .. } | Self::Dir { filelist, .. } => {
                filelist.gen_empty_pages(fname_padding)
            }

            Self::SingleImg { path } => vec![Page::new_empty(0)],

            Self::Unknown => eyre::bail!("Unknown Format"),
        })
    }

    pub fn get_file(&self, index: usize) -> eyre::Result<Vec<u8>> {
        match &self {
            Self::Archive { path, fmt, .. } => match fmt {
                ArchiveFmt::Zip => archive::zip::get_file(path, index),
                ArchiveFmt::Tar => archive::tar::get_file(path, index),
                _ => unreachable!(),
            },

            Self::Dir { path, .. } => archive::dir::get_file(path, index),

            Self::SingleImg { path } => archive::file::get_file(path),

            Self::Unknown => eyre::bail!("Unknown URI"),
        }
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

        let Some(val) = path.extension() else {
            return false;
        };
        let val = val.to_str().unwrap();

        SUPPORTED_FORMAT.contains(&val)
    }
}

// e.g. width = 6
//     01.jpg        -> 000001.jpg     (rename)
//     000001.jpg    -> 000001.jpg     (skip)
//     000000001.jpg -> 0000000001.jpg (skip)
pub fn padding_filename(width: usize, name: &str) -> String {
    let full = Path::new(name);

    let mut path = full.parent().unwrap().to_path_buf();
    let name = full.file_stem().unwrap().to_str().unwrap();
    let suffix = full.extension().unwrap().to_str().unwrap();

    // padding with `0`
    let name = format!("{0:0>width$}", name);
    let filename = format!("/{name}.{suffix}");
    path.push(PathBuf::from("filename").as_path());

    // tracing::debug!(path = path);

    path.display().to_string()
}
