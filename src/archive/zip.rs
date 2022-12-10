use crate::utils::err::{Res};


use std::{path::Path};


pub fn load_file<_Path>(path: &_Path, idx: usize) -> Res<Vec<u8>>
where
    _Path: AsRef<Path> + ?Sized,
{
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_zip")] {
            feat::load_file(path.as_ref(), idx)
        } else {
            Err(MyErr::FeatZip)
        }
    }
}

pub fn extract<_Path>(from: &_Path, to: &_Path) -> Res<()>
where
    _Path: AsRef<Path> + ?Sized,
{
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_zip")] {
            feat::extract(from.as_ref(), to.as_ref())
        } else {
            Err(MyErr::FeatZip)
        }
    }
}

pub fn get_file_list<_Path>(path: &_Path) -> Res<Vec<(String, usize)>>
where
    _Path: AsRef<Path> + ?Sized,
{
    cfg_if::cfg_if! {
        if #[cfg(feature = "ex_zip")] {
            feat::get_file_list(path.as_ref(), )
        } else {
            Err(MyErr::FeatZip)
        }
    }
}

#[cfg(feature = "ex_zip")]
mod feat {
    use crate::utils::err::{Res};
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::{fs::File, path::Path};
    use zip::ZipArchive;

    #[inline]
    pub fn extract<_Path>(zip_path: &_Path, target: &_Path) -> Res<()>
    where
        _Path: AsRef<Path> + ?Sized,
    {
        let mut zip = ZipArchive::new(File::open(zip_path.as_ref())?).unwrap();

        zip.extract(target.as_ref()).unwrap();

        Ok(())
    }

    #[inline]
    pub fn get_file_list<_Path>(path: &_Path) -> Res<Vec<(String, usize)>>
    where
        _Path: AsRef<Path> + ?Sized,
    {
        let mut res: Vec<(String, usize)> = Vec::new();

        let file = File::open(path.as_ref()).unwrap();
        let reader = BufReader::new(file);
        let mut zip = zip::ZipArchive::new(reader).unwrap();

        for idx in 0..zip.len() {
            let file = zip.by_index(idx).unwrap();

            res.push((file.name().to_string(), idx));
        }

        Ok(res)
    }

    #[inline]
    pub fn load_file<_Path>(path: &_Path, idx: usize) -> Res<Vec<u8>>
    where
        _Path: AsRef<Path> + ?Sized,
    {
        let file = File::open(path.as_ref()).unwrap();
        let reader = BufReader::new(file);
        let mut zip = zip::ZipArchive::new(reader).unwrap();
        let mut file = zip.by_index(idx).unwrap();

        let mut res = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut res).unwrap();

        Ok(res)
    }
}
