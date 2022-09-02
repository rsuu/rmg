use crate::utils::err::Res;
use std::{fs::File, io, path::Path};
use zstd;

pub fn extract<_Path>(from: &_Path, to: &_Path) -> Res<()>
where
    _Path: AsRef<Path> + ?Sized,
{
    let file = File::open(from.as_ref())?;

    let mut decoder = zstd::Decoder::new(file)?;
    let mut target = File::create(to.as_ref())?;

    io::copy(&mut decoder, &mut target)?;

    Ok(())
}

// REF
// https://github.com/gyscos/zstd-rs/blob/master/examples/zstd.rs
