use crate::Size;

pub fn load_aseprite(_bytes: &[u8]) -> eyre::Result<(Size, Vec<f32>, Vec<Vec<u8>>)> {
    #[cfg(feature = "de_aseprite")]
    {
        return feat::load_aseprite(_bytes);
    }

    eyre::bail!("")
}

#[cfg(feature = "de_aseprite")]
mod feat {
    use crate::Size;
    use asefile::AsepriteFile;
    use std::mem;

    #[inline]
    pub fn load_aseprite(bytes: &[u8]) -> eyre::Result<(Size, Vec<f32>, Vec<Vec<u8>>)> {
        let ase = AsepriteFile::read(bytes).unwrap();
        let size = Size::new(ase.width() as f32, ase.height() as f32);
        let head = 0;
        let tail = ase.num_frames();

        let mut data = Vec::with_capacity(tail as usize);
        let mut pts = Vec::with_capacity(tail as usize);
        for index in head..tail {
            let frame = ase.frame(index);

            pts.push(frame.duration() as f32);
            data.push(frame.image().to_vec());
        }

        Ok((size, pts, data))
    }
}
