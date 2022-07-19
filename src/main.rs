use cfg_if::cfg_if;
use emeta::meta;
use rmg::{
    archive::{tar, zip, zstd},
    cli,
    config::rsconf::Config,
    files::{self, list},
    img::size::{MetaSize, TMetaSize},
    reader::display,
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::TempDir;

#[tokio::main]
async fn main() {
    let tmp_dir = TempDir::new().unwrap().into_path();

    // parse config from file
    // OR
    // use default config
    let mut config: Config;
    //eprintln!("{:#?}", config);

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

        if let Some(path) = args.file_path {
            println!("Open: {}", path.as_str());

            // Copy files to temp_dir
            if let Ok(_) = open::<Path>(Path::new(path.as_str()), tmp_dir.as_path()) {
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

                // Only allow [.jpg || .jpeg || .png]
                let mut file_list: Vec<String> = walkdir::WalkDir::new(tmp_dir.as_path())
                    .into_iter()
                    .filter_map(|entry| {
                        let path = entry.as_ref().unwrap().path();
                        let path_name = path.to_str().unwrap();
                        if path.is_file() && path_name.ends_with(".jpg")
                            || path_name.ends_with(".png")
                            || path_name.ends_with(".jpeg")
                        {
                            Some(path_name.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                file_list.sort();
                //eprintln!("list: {:#?}", file_list);

                if config.base.rename {
                    if let Some(new) =
                        files::file::rename(false, args.rename_pad, file_list.as_slice())
                    {
                        file_list = new;
                    }
                } else {
                }

                // Vec<String> to Vec<&str>
                let file_list: Vec<&str> = file_list.iter().map(|s| s.as_str()).collect();
                //eprintln!("{:#?}", file_list);
                //eprintln!("{:#?}", config);
                match display::cat_img(
                    &config,
                    &file_list,
                    meta_size,
                    config.base.format,
                    &metadata,
                )
                .await
                {
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

    if fs::remove_dir_all(tmp_dir).is_err() {
        panic!()
    };
}

pub fn open<_Path>(from: &_Path, to: &_Path) -> Result<Vec<PathBuf>, std::io::Error>
where
    _Path: AsRef<Path> + ?Sized,
{
    if from.as_ref().is_dir() {
        files::dir::rec_copy_dir(from, to)?;
    } else {
        match list::get_filetype(from.as_ref()).as_str() {
            "tar" => {
                cfg_if! {
                    if #[cfg(feature="ex_zip")] {
                        tar::extract(from.as_ref(), to.as_ref())?;
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
                    if #[cfg(feature="ex_zip")] {
                        let _to = format!("{}/zstd.tar", to.as_ref().display());
                        zstd::extract(from.as_ref(), _to.as_ref()).unwrap();

                        let _from = _to;
                        tar::extract(_from.as_ref(), to.as_ref())?;

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
