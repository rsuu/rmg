use crate::img::Size;

pub fn load_ase(_bytes: &[u8]) -> anyhow::Result<(Size<u32>, Vec<Vec<u8>>, Vec<u32>)> {
    #[cfg(feature = "de_ase")]
    {
        return feat::load_ase(_bytes);
    }

    anyhow::bail!("")
}

#[cfg(feature = "de_ase")]
mod feat {
    use crate::{img::Size, FPS};
    use asefile::AsepriteFile;
    use std::mem;

    #[inline]
    pub fn load_ase(bytes: &[u8]) -> anyhow::Result<(Size<u32>, Vec<Vec<u8>>, Vec<u32>)> {
        let ase = AsepriteFile::read(bytes).unwrap();
        let size = Size::new(ase.width() as u32, ase.height() as u32);
        let head = 0;
        let tail = ase.num_frames();

        let mut data = Vec::with_capacity(tail as usize);
        let mut pts = Vec::with_capacity(tail as usize);
        for index in head..tail {
            let frame = ase.frame(index);

            pts.push(frame.duration());
            data.push(mem::take(&mut frame.image().to_vec()));
        }

        Ok((size, data, pts))
    }
}
