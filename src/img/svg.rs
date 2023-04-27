use crate::img::Size;

pub fn load_svg(_bytes: &[u8]) -> anyhow::Result<(Size<u32>, Vec<Vec<u8>>)> {
    #[cfg(feature = "de_svg")]
    {
        return feat::load_svg(_bytes);
    }

    anyhow::bail!("")
}

#[cfg(feature = "de_svg")]
mod feat {
    use crate::img::Size;

    pub fn load_svg(bytes: &[u8]) -> anyhow::Result<(Size<u32>, Vec<Vec<u8>>)> {
        use usvg::TreeParsing;

        // TODO: default (width, height)
        // BUG: font
        let opt = usvg::Options::default();
        let rtree = usvg::Tree::from_data(bytes, &opt).unwrap();
        let pixmap_size = rtree.size.to_screen_size();
        //.scale_to(usvg::ScreenSize::new(size.width, size.height).unwrap());

        let size = Size::new(pixmap_size.width(), pixmap_size.height());
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
