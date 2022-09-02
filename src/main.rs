use cfg_if::cfg_if;
use emeta::meta;
use log;
use rmg::{
    archive, cli,
    config::rsconf::Config,
    files::{self, list},
    img::size::{MetaSize, TMetaSize},
    reader::{
        self,
        buffer::{push_front, PageInfo},
        display,
    },
    utils::{err::MyErr, types::ArchiveType},
};
use simple_logger;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() {
    init();

    // parse config from file
    // OR
    // use default config
    let mut args = cli::parse::Args::new();

    args.parse().unwrap_or_else(|_| panic!());
    args.set_config_path();

    let mut config: Config = Config::default();

    match args.set_config_path() {
        Some(c) => config = c,
        None => {}
    }

    args.set_size(&mut config);

    log::debug!("Args: {:#?}", args);
    log::debug!("Config: {:#?}", config);

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
    log::debug!("meta_size: {:#?}", &meta_size);

    if let Some(path) = args.file_path {
        println!("Open: {}", path.as_str());

        let file_list = get_file_list(path.as_str()).unwrap();
        let mut page_list = if args.rename_pad == 0 {
            get_page_list(&file_list, 0)
        } else {
            get_page_list(&file_list, args.rename_pad)
        };
        let archive_type = get_archive_type(path.as_str()).unwrap();

        push_front(
            &mut page_list,
            &[PageInfo {
                path: PathBuf::new(),
                name: "".to_string(),
                len: 0,
                pos: 0,
            }],
        );

        log::debug!("page_list: {:?}", page_list);

        match display::cat_img(
            &config,
            page_list,
            meta_size,
            &None,
            path.as_str(),
            archive_type,
        )
        .await
        {
            Ok(_) => {
                std::process::exit(0);
            }
            Err(e) => match e {
                MyErr::Io(e) => {
                    panic!("{}", e);
                }
                _ => {}
            },
        }
    }
}

pub fn init() {
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
    let res: ArchiveType = if path.as_ref().is_dir() {
        ArchiveType::Dir
    } else {
        let inline_res: ArchiveType = match list::get_filetype(path.as_ref()).as_str() {
            "tar" => ArchiveType::Tar,
            "zip" => ArchiveType::Zip,

            _ => {
                return Err(());
            }
        };

        inline_res
    };

    Ok(res)
}

pub fn get_page_list(file_list: &[(String, usize)], rename_pad: usize) -> Vec<PageInfo> {
    let mut page_list = Vec::new();

    // Only allow [.jpg || .jpeg || .png]
    for (path, idx) in file_list.iter() {
        if !path.ends_with('/') && path.ends_with(".jpg")
            || path.ends_with(".png")
            || path.ends_with(".jpeg")
        {
            let info = if rename_pad == 0 {
                PageInfo::new(PathBuf::from(path.as_str()), path.clone(), 0, *idx)
            } else {
                PageInfo::new(
                    PathBuf::from(path.as_str()),
                    files::file::pad_name(rename_pad, path.as_str()),
                    0,
                    *idx,
                )
            };

            page_list.push(info);
        } else {
        }
    }

    page_list.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    log::debug!("sort: page_list: {:#?}", page_list.as_slice());

    page_list
}

pub fn get_file_list<_Path>(from: &_Path) -> Result<Vec<(String, usize)>, ()>
where
    _Path: AsRef<Path> + ?Sized,
{
    if from.as_ref().is_dir() {
        let file_list = archive::dir::get_file_list(from.as_ref()).unwrap();
        return Ok(file_list);
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
                        let file_list = archive::zip::get_file_list(from.as_ref()).unwrap();
                        return Ok(file_list);

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
