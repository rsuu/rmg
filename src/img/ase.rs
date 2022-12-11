use crate::{
    img::size::Size,
    utils::err::{MyErr, Res},
};

pub fn load_ase(bytes: &[u8]) -> Res<(Size<u32>, Vec<Vec<u8>>, Vec<u32>)> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "de_aseprite")] {
            feat::load_ase(bytes)
        } else {
           Err(MyErr::FeatAse)
        }
    }
}

#[cfg(feature = "de_aseprite")]
mod feat {
    use crate::{
        img::size::Size,
        reader::view::Page,
        utils::err::{MyErr, Res},
        FPS,
    };
    use asefile::AsepriteFile;
    use std::mem;

    #[inline]
    pub fn load_ase(bytes: &[u8]) -> Res<(Size<u32>, Vec<Vec<u8>>, Vec<u32>)> {
        let ase = AsepriteFile::read(bytes).unwrap();

        let size = Size::new(ase.width() as u32, ase.height() as u32);

        let head = 0;
        let tail = ase.num_frames();

        let mut data = Vec::with_capacity(tail as usize);
        let mut pts_list = Vec::with_capacity(tail as usize);

        for idx in head..tail {
            let mut frame = ase.frame(idx);
            pts_list.push(FPS as u32 + frame.duration());

            data.push(mem::take(&mut frame.image().to_vec()));
        }

        Ok((size, data, pts_list))
    }
}
