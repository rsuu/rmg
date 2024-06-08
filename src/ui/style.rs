use crate::*;

#[derive(Default, Clone)]
pub struct Style {
    size: Size,
    offset: Vec2,
    padding: Padding,
    margin: Margin,
}

#[derive(Default, Clone)]
pub struct Padding<T = f32> {
    top: T,
    right: T,
    buttom: T,
    left: T,
}

#[derive(Default, Clone)]
pub struct Margin<T = f32> {
    top: T,
    right: T,
    buttom: T,
    left: T,
}

pub enum AnimType {
    Ease,
}

impl Padding {
    pub fn new(top: f32, right: f32, buttom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            buttom,
            left,
        }
    }
}

impl Margin {
    pub fn new(top: f32, right: f32, buttom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            buttom,
            left,
        }
    }
}
