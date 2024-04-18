use crate::*;

pub fn load_svg(_bytes: &[u8]) -> eyre::Result<(Size, Vec<Vec<u8>>)> {
    #[cfg(feature = "de_svg")]
    {
        return feat::load_svg(_bytes);
    }

    eyre::bail!("")
}

#[cfg(feature = "de_svg")]
mod feat {
    use crate::img::Size;

    pub fn load_svg(bytes: &[u8]) -> eyre::Result<(Size, Vec<Vec<u8>>)> {
        use usvg::TreeParsing;

        // TODO: default (width, height)
        // window w,h
        // BUG: font
        let opt = usvg::Options::default();
        let rtree = usvg::Tree::from_data(bytes, &opt).unwrap();
        let pixmap_size = rtree.size.to_screen_size();
        //.scale_to(usvg::ScreenSize::new(size.width, size.height).unwrap());

        let size = Size::from_u32(pixmap_size.width(), pixmap_size.height());
        //let mut pixmap = tiny_skia::Pixmap::new(size.width, size.height).unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

        resvg::render(
            &rtree,
            resvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .unwrap();

        Ok((size, vec![pixmap.data().to_vec()]))
    }
}
