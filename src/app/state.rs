use crate::*;

use esyn::EsynDe;
use rgb::RGBA8;

/// NOTE: Only works in [Layout::Double]
// REFS: https://developer.mozilla.org/en-US/docs/Web/CSS/direction
#[derive(Debug, Default, Clone, Copy, PartialEq, EsynDe)]
pub enum PageDirection {
    // Right to Left
    #[default]
    Manga,
    // Left to Right
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
    // https://docs.rs/winit/0.30.0/winit/event/enum.Ime.html
    // https://github.com/alacritty/alacritty/blob/38fed9a7c233e11e5f62433298235281fc3de885/alacritty/src/display/mod.rs#L1062
    // IME
}
