use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct ViewArea<T = Rect> {
    pub view: T,
    pub border: T,
}

impl ViewArea {
    pub fn is_page_hover(&self, page: &Page) -> (bool, bool) {
        // dbg!(&self, page.cast_vertex);

        let Self { view, border } = *self;
        let page = &page.cast_vertex;

        let is_hover_edge = {
            if border.is_include(page.min()) || border.is_include(page.max()) {
                true
            } else {
                false
            }
        };
        let is_hover_view = {
            // if page.max().y >= view.min().y && page.min().y <= view.max().y {
            if view.is_include(page.min()) || view.is_include(page.max()) {
                true
            } else {
                false
            }
        };

        // dbg!((page, border, view));

        (is_hover_edge, is_hover_view)
    }
}
