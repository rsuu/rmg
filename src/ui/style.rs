#[derive(Default, Clone)]
pub struct Style {
    padding: Padding,
    margin: Margin,
}

#[derive(Default, Clone)]
pub struct Padding<T = u32> {
    top: T,
    right: T,
    buttom: T,
    left: T,
}

#[derive(Default, Clone)]
pub struct Margin<T = u32> {
    top: T,
    right: T,
    buttom: T,
    left: T,
}

pub enum Animation {
    Ease,
}

impl Padding {
    pub fn new(top: u32, right: u32, buttom: u32, left: u32) -> Self {
        Self {
            top,
            right,
            buttom,
            left,
        }
    }
}

impl Margin {
    pub fn new(top: u32, right: u32, buttom: u32, left: u32) -> Self {
        Self {
            top,
            right,
            buttom,
            left,
        }
    }
}
