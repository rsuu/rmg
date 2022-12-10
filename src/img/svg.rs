use resvg;
use tiny_skia;
use usvg;

/*

pub fn load_svg(bytes: &[u8]) -> Res<(Size<u32>, Vec<Vec<u8>>)> {
    // let (width, height) = (3000, 3000);
    let opt = usvg::Options::default();
    let svg_data = std::fs::read(&img_location)?;
    if let Ok(rtree) = usvg::Tree::from_data(&svg_data, &opt.to_ref()) {
        // let pixmap_size = rtree.svg_node().size.to_screen_size()
        let pixmap_size = rtree.size.to_screen_size();
        // .scale_to(ScreenSize::new(width, height)?);

        if let Some(mut pixmap) = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        {
            resvg::render(
                &rtree,
                usvg::FitTo::Original,
                tiny_skia::Transform::identity(),
                pixmap.as_mut(),
            )
            .ok_or(anyhow!("Can't render SVG"))?;
            // resvg::render(&rtree, usvg::FitTo::Height(height), pixmap.as_mut())?;
            let buf: Option<RgbaImage> = image::ImageBuffer::from_raw(
                pixmap_size.width(),
                pixmap_size.height(),
                pixmap.data().to_vec(),
            );
            if let Some(valid_buf) = buf {
                col.add_still(valid_buf);
            }
        }
    }
}

*/
