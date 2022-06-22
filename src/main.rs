use std::{
    fs,
    path::{Path, PathBuf},
};

use tempfile::TempDir;

use rmg::{
    archive::{tar, zip},
    cli,
    files::list,
    img::size::{Size, TMetaSize},
    reader::display,
};

fn main() {
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
                let file_list: Vec<&str> = list.iter().map(|s| s.to_str().unwrap()).collect();
                display::cat_rgb_img(file_list.as_slice(), window_size).unwrap();
            }
        }
    }

    if fs::remove_dir_all(tmp_dir).is_err() {
        panic!()
    };
}

pub fn open(path: &Path, tmp_dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let ty = match list::get_filetype(path).as_ref() {
        "tar" => {
            tar::extract(path, tmp_dir)?;
        }

        "zip" => {
            zip::extract(path, tmp_dir)?;
        }

        _ => panic!(),
    };

    Ok(list::get_file_list(tmp_dir))
}
