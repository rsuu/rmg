use cfg_if::cfg_if;
use emeta::meta;
use log::{debug, error, info, trace, warn};
use rmg::{
    archive::{self, zip},
    cli,
    config::rsconf::Config,
    files::{self, list},
    img::size::{MetaSize, TMetaSize},
    reader::{
        buffer::{self, PageInfo},
        display,
    },
    utils::types::ArchiveType,
};
use simple_logger;
use std::ptr;
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};
use tempfile::TempDir;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum DataStoreError {
    #[error("I/O")]
    Disconnect(#[from] std::io::Error),

    #[error("unknown")]
    Unknown,
}

fn main() {
    //env_logger::init();
    init();

    // parse config from file
    // OR
    // use default config
    let mut config: Config;

    if let Ok(args) = cli::parse::Args::get_args() {
        if let Some(config_path) = args.config_path {
            config = Config::parse_from(config_path.as_str());
        } else {
            config = Config::parse_from("./tests/files/config.rs");
        }

        if let Some(size) = args.size {
            config.base.size = size;
        } else {
            // default
        };

        let meta_size = MetaSize::new(
            0,
            0,
            config.base.size.width as u32,
            config.base.size.height as u32,
            0,
            0,
        );
        debug!("meta_size: {:#?}", &meta_size);

        if let Some(path) = args.file_path {
            println!("Open: {}", path.as_str());

            let file_list = get_file_list(path.as_str()).unwrap();
            let page_list = if config.base.rename {
                get_page_list(&file_list, args.rename_pad)
            } else {
                get_page_list(&file_list, 0)
            };

            match display::cat_img(
                &config,
                page_list,
                meta_size,
                config.base.format,
                &None,
                path.as_str(),
            ) {
                Ok(_) => {}
                Err(e) => match e {
                    rmg::utils::types::MyError::ErrIo(e) => {
                        panic!("{}", e);
                    }
                    _ => {}
                },
            }
        } else {
            println!("err");
        }
    }
}

fn init() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Off)
        .with_colors(true)
        .without_timestamps()
        .env()
        .init()
        .unwrap();
}

pub fn get_archive_type<_Path>(path: &_Path) -> Result<ArchiveType, ()>
where
    _Path: AsRef<Path> + ?Sized,
{
    let mut ty: ArchiveType = ArchiveType::Tar;
    todo!();
}

pub fn get_page_list(file_list: &[String], rename_pad: usize) -> Vec<PageInfo> {
    let mut page_list = Vec::new();

    // Only allow [.jpg || .jpeg || .png]
    for (pos, p) in file_list.iter().enumerate() {
        if !p.ends_with('/') && p.ends_with(".jpg") || p.ends_with(".png") || p.ends_with(".jpeg") {
            let p = if rename_pad == 0 {
                PageInfo::new(PathBuf::from(&p), p.clone(), 0, pos)
            } else {
                PageInfo::new(
                    PathBuf::from(&p),
                    files::file::pad_name(rename_pad, p.as_str()),
                    0,
                    pos,
                )
            };

            page_list.push(p);
        } else {
        }
    }

    page_list.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    debug!("sort: page_list: {:#?}", page_list.as_slice());

    page_list
}

pub fn get_file_list<_Path>(from: &_Path) -> Result<Vec<String>, ()>
where
    _Path: AsRef<Path> + ?Sized,
{
    if from.as_ref().is_dir() {
        //files::dir::rec_copy_dir(from, to)?;
        return Err(());
    } else {
        match list::get_filetype(from.as_ref()).as_str() {
            "tar" => {
                cfg_if! {
                    if #[cfg(feature="ex_tar")] {
                        let file_list = archive::tar::get_file_list(from.as_ref()).unwrap();
                        return Ok(file_list);

                    } else {
                        eprintln!("Not Support FileType: tar");
                        return Err(());

                    }
                }
            }
            "zip" => {
                cfg_if! {
                    if #[cfg(feature="ex_zip")] {
                        println!("Open zip");
                        return Err(());

                    }else {
                        eprintln!("Not Support FileType: zip");
                        return Err(());
                    }
                }
            }

            _ => {
                return Err(());
            }
        };
    };
}

/*
pub fn load_meta<_Path>(from: &_Path, to: &_Path) -> Result<Vec<PathBuf>, std::io::Error>
where
    _Path: AsRef<Path> + ?Sized,
{
    if from.as_ref().is_dir() {
        files::dir::rec_copy_dir(from, to)?;
    } else {
        match list::get_filetype(from.as_ref()).as_str() {
            "tar" => {
                cfg_if! {
                    if #[cfg(feature="ex_tar")] {
                        //archive::tar::extract(from.as_ref(), to.as_ref())?;
                    } else {
                        eprintln!("Not Support FileType: tar");
                    }
                }
            }

            "zip" => {
                cfg_if! {
                    if #[cfg(feature="ex_zip")] {
                        println!("Open zip");
                        zip::extract(from.as_ref(), to.as_ref())?;
                    }else {
                        eprintln!("Not Support FileType: zip");
                    }
                }
            }

            "zst" => {
                cfg_if! {
                    if #[cfg(feature="ex_zstd")] {
                        let _to = format!("{}/zstd.tar", to.as_ref().display());
                        zstd::extract(from.as_ref(), _to.as_ref()).unwrap();

                        let _from = _to;
                       archive::tar::extract(_from.as_ref(), to.as_ref())?;

                        fs::remove_file(_from)?;
                    }else {
                        eprintln!("Not Support FileType: zstd");
                    }
                }
            }

            _ => panic!(),
        };
    }
    Ok(list::get_file_list(to.as_ref()))
}


           // Copy files to temp_dir
           if let Ok(_) = open::<Path>(Path::new(path.as_str())) {
               // Check if has ".rmg" file
               let mut rmg_file: Option<String> = None;
               let mut metadata: Option<meta::MetaData> = None;

               for f in walkdir::WalkDir::new(tmp_dir.as_path()).into_iter() {
                   if f.as_ref().expect("").path().ends_with(".rmg") {
                       rmg_file = Some(f.as_ref().expect("").path().display().to_string());
                       break;
                   } else {
                   }
               }

               if let Some(rmg_path) = rmg_file {
                   metadata = Some(meta::MetaData::from_file(rmg_path.as_str()).unwrap());

                   // e.g. rmg xxx.tar --meta d
                   if args.meta_display {
                       metadata.as_ref().unwrap().display();
                       std::process::exit(0); // EXIT
                   } else {
                   }
               } else {
               }

*/
