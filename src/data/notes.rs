// TODO: notes
// TODO: text

use crate::*;

use rgb::RGBA8;

#[derive(Debug)]
pub struct Notes {
    inner: Vec<Note>,
}

#[derive(Debug)]
pub struct Note {
    pub color: RGBA8,
    pub data: Data,
}

#[derive(Debug)]
enum Data {
    Shape {
        stroke_width: u8,
        points: Vec<Vec2>,
    },
    Text {
        content: String,
        font: Font,
        start_at: Vec2,
    },
    RichText {
        // ?
    },
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

impl Note {
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
