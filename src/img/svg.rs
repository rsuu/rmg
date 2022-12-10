use resvg;
use tiny_skia;
use usvg::{self, ScreenRect};

use crate::utils::err::Res;

use super::size::Size;

pub fn load_svg(bytes: &[u8]) -> Res<(Size<u32>, Vec<Vec<u8>>)> {
    let size = Size::new(2000, 2000);
    let opt = usvg::Options::default();

    if let Ok(rtree) = usvg::Tree::from_data(&bytes, &opt.to_ref()) {
        let pixmap_size = rtree
            .size
            .to_screen_size()
            .scale_to(usvg::ScreenSize::new(size.width, size.height).unwrap());

        if let Some(mut pixmap) = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        {
            resvg::render(
                &rtree,
                usvg::FitTo::Height(size.height),
                //usvg::FitTo::Original,
                tiny_skia::Transform::identity(),
                pixmap.as_mut(),
            )
            .unwrap();

            return Ok((size, vec![pixmap.data().to_vec()]));
        } else {
        }
    } else {
    }

    Err(crate::utils::err::MyErr::Todo)
}
