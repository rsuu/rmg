use crate::{archive::*, Path};

pub fn get_file<P>(path: &P, index: usize) -> eyre::Result<Vec<u8>>
where
    P: AsRef<Path> + ?Sized,
{
    #[cfg(feature = "ex_zip")]
    {
        return feat::get_file(path.as_ref(), index);
    }

    eyre::bail!("")
}

pub fn get_list<P>(path: &P) -> eyre::Result<FileList>
where
    P: AsRef<Path> + ?Sized,
{
    #[cfg(feature = "ex_zip")]
    {
        return feat::get_list(path.as_ref());
    }

    eyre::bail!("")
}

pub fn extract<P>(from: &P, to: &P) -> eyre::Result<()>
where
    P: AsRef<Path> + ?Sized,
{
    #[cfg(feature = "ex_zip")]
    {
        return feat::extract(from.as_ref(), to.as_ref());
    }

    eyre::bail!("")
}

#[cfg(feature = "ex_zip")]
mod feat {
    extern crate zip;
    use crate::archive::*;
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
    };

    pub fn extract<P>(zip_path: &P, target: &P) -> eyre::Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let mut zip = zip::ZipArchive::new(File::open(zip_path.as_ref())?)?;

        zip.extract(target.as_ref())?;

        Ok(())
    }

    pub fn get_list<P>(path: &P) -> eyre::Result<FileList>
    where
        P: AsRef<Path> + ?Sized,
    {
        let mut res = FileList::new();

        let file = File::open(path.as_ref())?;
        let render = BufReader::new(file);
        let mut zip = zip::ZipArchive::new(render)?;

        for index in 0..zip.len() {
            let file = zip.by_index(index)?;

            res.push(FileInfo::new(file.name().to_string(), index));
        }

        Ok(res)
    }

    pub fn get_file<P>(path: &P, index: usize) -> eyre::Result<Vec<u8>>
    where
        P: AsRef<Path> + ?Sized,
    {
        let file = File::open(path.as_ref())?;
        let render = BufReader::new(file);
        let mut zip = zip::ZipArchive::new(render)?;
        let mut file = zip.by_index(index)?;

        let mut res = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut res)?;

        Ok(res)
    }
}
