use crate::*;

use rgb::RGBA8;

#[derive(Debug)]
pub struct Nav {
    pub align: Align,
}

#[derive(Debug, Default)]
pub struct LeaveBlank {
    pub rect: Rect,
    pub size: Size,
    pub bg: RGBA8,
}

impl Nav {
    pub fn new(align: Align) -> Self {
        Self { align }
    }
}
