use std::{
    fs,
    path::{Path, PathBuf},
};

use tempfile::TempDir;

use rmg::{
    archive::{tar, zip},
    cli,
    config::rsconf::Config,
    files::{self, list},
    img::size::{MetaSize, TMetaSize},
    reader::display,
};

#[tokio::main]
async fn main() {
    let tmp_dir = TempDir::new().unwrap().into_path();

    // parse config from file
    // OR
    // use default config
    let mut config = Config::parse_from("./tests/files/config.rs");
    //eprintln!("{:#?}", config);

    if let Ok(args) = cli::parse::Args::get_args() {
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

                for f in walkdir::WalkDir::new(tmp_dir.as_path()).into_iter() {
                    if f.as_ref().unwrap().path().ends_with(".rmg") {
                        rmg_file = Some(f.as_ref().unwrap().path().display().to_string());
                        break;
                    } else {
                    }
                }

                if rmg_file.is_some() {
                    eprintln!("rmg_file");
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

                eprintln!("{:#?}", config);
                match display::cat_img(&config, &file_list, meta_size, config.base.format).await {
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

pub fn open<T>(from: &T, to: &T) -> Result<Vec<PathBuf>, std::io::Error>
where
    T: AsRef<Path> + ?Sized,
{
    if from.as_ref().is_dir() {
        files::dir::rec_copy_dir(from, to)?;
    } else {
        match list::get_filetype(from.as_ref()).as_str() {
            "tar" => {
                tar::extract(from.as_ref(), to.as_ref())?;
            }

            "zip" => {
                //println!("Open zip");
                zip::extract(from.as_ref(), to.as_ref())?;
            }

            _ => panic!(),
        };
    }

    Ok(list::get_file_list(to.as_ref()))
}
