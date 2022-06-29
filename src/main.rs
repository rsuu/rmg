use std::{
    fs,
    path::{Path, PathBuf},
};

use tempfile::TempDir;

use rmg::{
    archive::{tar, zip},
    cli,
    files::list,
    img::size::Size,
    reader::display,
};

#[tokio::main]
async fn main() {
    let tmp_dir = TempDir::new().unwrap().into_path();

    if let Ok(args) = cli::parse::Args::get_args() {
        let window_size = if let Some(size) = args.size {
            size
        } else {
            Size::<u32> {
                width: 1440,
                height: 900,
            }
        };

        if let Some(path) = args.file_path {
            println!("Open: {}", path.as_str());

            if let Ok(list) = open(Path::new(path.as_str()), tmp_dir.as_path()) {
                // Only allow [.jpg || .jpeg || .png]
                let file_list: Vec<String> = walkdir::WalkDir::new(tmp_dir.as_path())
                    .into_iter()
                    .filter_map(|entry| {
                        let entry = entry.unwrap();
                        let path = entry.path();
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

                let mut file_list: Vec<&str> = file_list.iter().map(|s| s.as_str()).collect();
                file_list.sort();
                println!("list: {:?}", file_list);

                match display::cat_rgb_img(&file_list, window_size).await {
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

pub fn open(from: &Path, to: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    match list::get_filetype(from).as_ref() {
        "tar" => {
            tar::extract(from, to)?;
        }

        "zip" => {
            //println!("Open zip");
            zip::extract(from, to)?;
        }

        _ => panic!(),
    };

    Ok(list::get_file_list(to))
}
