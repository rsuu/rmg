use crate::*;
use esyn::EsynDe;
use rgb::RGBA8;

#[derive(Debug, Default, Clone, Copy, PartialEq, EsynDe)]
pub enum Mode {
    #[default]
    Manga,
    Comic,
}

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    View,
    Gesture {
        // static
        fill: RGBA8,
        stroke_width: f32,
        // filter: // ?grayscale

        // mut
        path: Vec<Vec2>,
    },
}
