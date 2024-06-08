// TODO: notes
// TODO: text
// TODO: https://github.com/flxzt/rnote/issues/15

use crate::*;

use rgb::RGBA8;

/// All points in normalized.
// REFS: https://github.com/flxzt/rnote/blob/205223c078fa1254062b956a2cf6697aca5347b3/crates/rnote-engine/src/fileformats/xoppformat.rs#L432
#[derive(Debug)]
pub struct Notes {
    pages: Vec<NotePage>,
}

#[derive(Debug)]
pub struct NotePage {
    pub size: Size,
    pub page_number: usize,
    // pub background: XoppBackground,
    /// The layers of the page.
    pub layers: Vec<NoteLayer>,
}

#[derive(Debug)]
pub struct NoteLayer {
    /// Stroke on this layer.
    pub strokes: Vec<NoteStroke>,
    /// Texts on this layer.
    pub texts: Vec<NoteText>,
    // /// Images on this layer.
    // pub images: Vec<XoppImage>,
}

#[derive(Debug)]
pub struct NoteStroke {
    color: RGBA8,
    tool: ToolType,
    fill: Option<RGBA8>,
    start_at: Vec2,
}

#[derive(Debug)]
pub enum ToolType {
    Pixel {
        stroke_width: f32,
        points: Vec<Vec2>,
    },
    Rect {
        size: Size,
    },
    Circle {
        radius: f32,
    },
}

#[derive(Debug)]
pub struct NoteText {
    content: String,
    font: Font,
    color: RGBA8,
    start_at: Vec2,
}

// REFS: https://developer.mozilla.org/en-US/docs/Web/CSS/font
#[derive(Debug)]
struct Font {
    font_family: String,
    font_style: String,
    font_variant: String,

    font_size: u8,
    font_weight: u8,
    line_height: u8,
    font_stretch: u8,
}

impl NoteLayer {
    pub fn new(size: Size, color: RGBA8, path: &[Vec2]) -> Self {
        let mut res: Vec<Vec2> = Vec::new();
        let len = size.len();

        for p in path.iter() {
            // res.push(p.normalized(len));
        }

        todo!();
    }

    // mixing color
}
