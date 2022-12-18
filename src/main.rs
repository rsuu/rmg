use rmg::{
    archive::{self, ArchiveType},
    config::rsconf::Config,
    img::size::{MetaSize, TMetaSize},
    reader::{display, view::Page},
    utils::{cli, err::MyErr, file},
};

use std::path::Path;

fn main() {
    init_log();

    let mut args = cli::Args::new();
    let mut config: Config = args.init_config();

    args.parse(&mut config).unwrap_or_else(|_| todo!());

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

    let Some(path) = &config.cli.file_path else { rmg::utils::cli::print_help() };

    println!("Open: {}", path.as_str());

    let archive_type = get_archive_type(path.as_str());
    let file_list = get_file_list(path.as_str()).unwrap();
    let mut page_list = get_page_list(&file_list, config.base.rename_pad as usize);

    log::debug!("file_list: {:#?}", file_list);
    log::debug!("page_list: {:#?}", page_list);

    match display::cat_img(
        &config,
        &mut page_list,
        meta_size,
        path.as_str(),
        archive_type,
    ) {
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

pub fn init_log() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Off)
        .with_colors(true)
        .without_timestamps()
        .env()
        .init()
        .unwrap();
}

pub fn get_archive_type<_Path>(path: &_Path) -> ArchiveType
where
    _Path: AsRef<Path> + ?Sized,
{
    if path.as_ref().is_dir() {
        ArchiveType::Dir
    } else {
        match file::get_filetype(path.as_ref()).as_str() {
            "tar" => ArchiveType::Tar,
            "zip" => ArchiveType::Zip,

            _ => ArchiveType::File,
        }
    }
}

pub fn get_page_list(file_list: &[(String, usize)], rename_pad: usize) -> Vec<Page> {
    let mut page_list = Vec::new();

    for (path, pos) in file_list.iter() {
        if rmg::utils::file::has_supported(AsRef::<Path>::as_ref(path.as_str())) {
            let page = if rename_pad == 0 {
                Page::new(path.clone(), *pos)
            } else {
                // rename
                Page::new(file::pad_name(rename_pad, path.as_str()), *pos)
            };

            page_list.push(page);
        } else {
            // skip
        }
    }

    page_list
}

pub fn get_file_list(path: impl AsRef<Path>) -> Result<Vec<(String, usize)>, ()> {
    if path.as_ref().is_dir() {
        let file_list = archive::dir::get_file_list(path.as_ref()).unwrap();
        Ok(file_list)
    } else {
        match file::get_filetype(path.as_ref()).as_str() {
            "tar" => {
                let Ok(file_list) = archive::tar::get_file_list(path.as_ref()) else {
                    return Err(());
                };

                Ok(file_list)
            }

            "zip" => {
                if let Ok(file_list) = archive::zip::get_file_list(path.as_ref()) {
                    Ok(file_list)
                } else {
                    Err(())
                }
            }

            _ => {
                // e.g. rmg test.gif
                let ty = file::get_filetype(path.as_ref());

                println!("{}", ty);

                if rmg::reader::view::ImgFormat::from(ty.as_str())
                    == rmg::reader::view::ImgFormat::Unknown
                {
                    // unsupport
                    Err(())
                } else {
                    // Only one file.
                    return Ok(vec![(path.as_ref().display().to_string(), 0)]);
                }
            }
        }
    }
}
