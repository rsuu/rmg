use cfg_if::cfg_if;

use log;
use rmg::{
    archive, cli,
    config::rsconf::Config,
    files::{self, list},
    img::size::{MetaSize, TMetaSize},
    reader::{buffer::PageInfo, display},
    utils::{err::MyErr, types::ArchiveType},
};
use simple_logger;
use std::path::Path;

#[tokio::main]
async fn main() {
    init();

    let mut args = cli::parse::Args::new();
    args.parse().unwrap_or_else(|_| panic!());

    let mut config: Config = args.init_config();
    args.set_size(&mut config);

    log::debug!("Args: {:#?}", args);
    log::debug!("Config: {:#?}", config);

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
        let archive_type = get_archive_type(path.as_str()).unwrap();
        let page_list = get_page_list(&file_list, args.rename_pad);

        log::debug!("page_list: {:#?}", page_list);

        match display::cat_img(
            &config,
            page_list,
            meta_size,
            //&None,
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

    // Only allow [.jpg || .jpeg || .png || .avif]
    for (path, idx) in file_list.iter() {
        if !path.ends_with('/') && path.ends_with(".jpg")
            || path.ends_with(".png")
            || path.ends_with(".jpeg")
            || path.ends_with(".avif")
            || path.ends_with(".heic")
            || path.ends_with(".heif")
        {
            let info = if rename_pad == 0 {
                PageInfo::new(path.clone(), *idx)
            } else {
                PageInfo::new(files::file::pad_name(rename_pad, path.as_str()), *idx)
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
