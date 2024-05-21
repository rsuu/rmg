use crate::*;

#[derive(Clone)]
pub struct Buffer {
    pub vec: Vec<u32>,
    pub size: Size,
}

impl Buffer {
    pub fn new(vec: Vec<u32>, size: Size) -> Self {
        Self { vec, size }
    }

    pub fn bytes(&self) -> &[u32] {
        &self.vec
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
