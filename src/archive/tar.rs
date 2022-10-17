use crate::utils::err::{MyErr, Res};
use std::{
    fs::{File, OpenOptions},
    io::Read,
    path::Path,
};

pub fn load_file(path: &Path, idx: usize) -> Res<Vec<u8>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_tar")] {
            feat::load_file(path.as_ref(), idx)
        } else {
            Err(MyErr::FeatTar)
        }
    }
}

// feat! {
//   ok {
//     #[cfg(feature = "ex_tar")]
//   }
//
//   err {}
// }

// #[allow(unused)]
// pub fn extract(tar_path: &Path, tmp_dir: &Path) -> Result<(), std::io::Error> {
//     let file = File::open(tar_path)?;
//     let mut tar = tar::Archive::new(file);
//     tar.unpack(tmp_dir)?;
//
//     Ok(())
// }

pub fn get_file_list(path: &Path) -> Res<Vec<(String, usize)>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_tar")] {
            feat::get_file_list(path.as_ref())
        } else {
            Err( MyErr::FeatTar)
        }
    }
}

#[cfg(feature = "ex_tar")]
mod feat {
    use crate::utils::err::{MyErr, Res};
    use std::{
        fs::{File, OpenOptions},
        io::Read,
        path::Path,
    };
    use tar;

    #[inline]
    pub fn get_file_list(path: &Path) -> Res<Vec<(String, usize)>> {
        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(path)?;
        let mut tar = tar::Archive::new(file);
        let mut list = Vec::new();

        for (idx, f) in tar.entries()?.enumerate() {
            list.push((f?.path()?.display().to_string(), idx));
        }

        Ok(list)
    }

    #[inline]
    pub fn load_file(tar_file: &Path, idx: usize) -> Res<Vec<u8>> {
        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(tar_file)
            .unwrap();
        let mut tar_file = tar::Archive::new(&file);
        let mut buffer = Vec::new();

        for (n, file) in tar_file.entries().unwrap().enumerate() {
            if n == idx {
                let mut f = file.unwrap();
                f.read_to_end(&mut buffer).expect("");

                return Ok(buffer);
            } else {
            }
        }

        Err(MyErr::Null(()))
    }
}
