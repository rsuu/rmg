use log;
use rmg::{
    archive::{self, ArchiveType},
    config::rsconf::Config,
    img::size::{MetaSize, TMetaSize},
    reader::{
        display,
        view::{Img, Page},
    },
    utils::{cli, err::MyErr, file},
};
use simple_logger;
use std::path::Path;

#[tokio::main]
async fn main() {
    init_log();

    let mut args = cli::Args::new();
    let mut config: Config = args.init_config();

    args.parse(&mut config).unwrap_or_else(|_| panic!());

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

    if let Some(path) = &config.cli.file_path {
        println!("Open: {}", path.as_str());

        let file_list = get_file_list(path.as_str()).unwrap();
        let archive_type = get_archive_type(path.as_str()).unwrap();
        let page_list = get_page_list(&file_list, config.base.rename_pad as usize);

        log::debug!("page_list: {:#?}", page_list);

        match display::cat_img(&config, page_list, meta_size, path.as_str(), archive_type).await {
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

pub fn init_log() {
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
        let inline_res: ArchiveType = match file::get_filetype(path.as_ref()).as_str() {
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

pub fn get_page_list(file_list: &[(String, usize)], rename_pad: usize) -> Vec<Page> {
    let mut page_list = Vec::new();
    let mut number = 0;

    // Only allow [.jpg || .jpeg || .png || .avif]
    for (path, pos) in file_list.iter() {
        if rmg::has_supported(path.as_str()) {
            let info = if rename_pad == 0 {
                Page::new(path.clone(), number, *pos)
            } else {
                Page::new(file::pad_name(rename_pad, path.as_str()), number, *pos)
            };

            page_list.push(info);
            number += 1;
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
        match file::get_filetype(from.as_ref()).as_str() {
            "tar" => {
                if let Ok(file_list) = archive::tar::get_file_list(from.as_ref()) {
                    return Ok(file_list);
                } else {
                    return Err(());
                }
            }

            "zip" => {
                if let Ok(file_list) = archive::zip::get_file_list(from.as_ref()) {
                    return Ok(file_list);
                } else {
                    return Err(());
                }
            }

            _ => {
                return Err(());
            }
        };
    };
}
