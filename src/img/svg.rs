use crate::{
    img::size::Size,
    utils::err::{MyErr, Res},
};

pub fn load_svg(_bytes: &[u8]) -> Res<(Size<u32>, Vec<Vec<u8>>)> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "de_svg")] {
            feat::load_svg(_bytes)
        } else {
           Err(MyErr::FeatSvg)
        }
    }
}

#[cfg(feature = "de_svg")]
mod feat {
    use crate::{
        img::size::Size,
        utils::err::{MyErr, Res},
    };

    pub fn load_svg(bytes: &[u8]) -> Res<(Size<u32>, Vec<Vec<u8>>)> {
        // BUG:
        let mut opt = usvg::Options::default();
        let rtree = usvg::Tree::from_data(bytes, &opt).unwrap();
        let pixmap_size = rtree.size.to_screen_size();
        //.scale_to(usvg::ScreenSize::new(size.width, size.height).unwrap());

        let size = Size::new(pixmap_size.width(), pixmap_size.height());
        //let mut pixmap = tiny_skia::Pixmap::new(size.width, size.height).unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

        resvg::render(
            &rtree,
            usvg::FitTo::Original,
            //usvg::FitTo::Height(size.height),
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .unwrap();

        Ok((size, vec![pixmap.data().to_vec()]))
    }
}
