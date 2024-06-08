use crate::*;

use esyn::EsynDe;
use rgb::RGBA8;

/// NOTE: Only works in [Layout::Double]
// REFS: https://developer.mozilla.org/en-US/docs/Web/CSS/direction
#[derive(Debug, Default, Clone, Copy, PartialEq, EsynDe)]
pub enum Direction {
    // Right to Left
    #[default]
    Rtl,
    // Left to Right
    Ltr,
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

    // TODO:
    // 1. toggle selection mode
    //    if Key::S {
    //        match self.atcion
    //            FreehandSelection => self.action = Action::View
    //            _ => self.action = FreehandSelection
    //    }
    //    // Key::S
    // 2. if action == FreehandSelection && left-click() {
    //        vertex.push(mouse-pos);
    //    }
    // 3. draw
    //    for [a, b] in vertex.as_slice().windows(2) {
    //        draw_line(a, b, RED);
    //    }
    //    // preview
    //    if mouse-pos != vertex.last() {
    //        draw_line(mouse-pos, vertex.last(), GRAY);
    //    }
    // REFS: https://docs.krita.org/reference_manual/tools/freehand_select.html
    FreehandSelection {
        vertex: Vec<Vec2>,
    },
    // https://docs.rs/winit/0.30.0/winit/event/enum.Ime.html
    // https://github.com/alacritty/alacritty/blob/38fed9a7c233e11e5f62433298235281fc3de885/alacritty/src/display/mod.rs#L1062
    // IME
    // TextInput {}
}
