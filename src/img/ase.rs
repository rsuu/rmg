use crate::{
    img::size::Size,
    utils::err::{MyErr, Res},
};

pub fn load_ase(bytes: &[u8]) -> Res<(Size<u32>, Vec<Vec<u8>>)> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "de_ase")] {
            feat::load_ase(bytes)
        } else {
           Err(MyErr::FeatAse)
        }
    }
}

#[cfg(feature = "de_ase")]
mod feat {
    use crate::{
        img::size::Size,
        reader::view::Page,
        utils::err::{MyErr, Res},
    };
    use asefile::AsepriteFile;

    #[inline]
    pub fn load_ase(bytes: &[u8]) -> Res<(Size<u32>, Vec<Vec<u8>>)> {
        let ase = AsepriteFile::read(bytes).unwrap();

        let size = Size::new(ase.width() as u32, ase.height() as u32);

        let head = 0;
        let tail = ase.num_frames();

        let mut data = Vec::with_capacity(tail as usize);

        for idx in head..tail {
            let frame = ase.frame(idx);

            data.push(frame.image().to_vec());
        }

        Ok((size, data))
    }
}
