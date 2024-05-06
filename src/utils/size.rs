use std::ops::Mul;

use esyn::EsynDe;

#[derive(Debug, Clone, Copy, Default, EsynDe, PartialEq, Eq)]
pub struct Size<T = f32> {
    pub width: T,
    pub height: T,
}

#[derive(Debug, Clone, Copy)]
pub struct MetaSize<T = f32> {
    pub window: Size<T>,
    pub canvas: Size<T>,
}

// ==============================================
impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Size { width, height }
    }

    pub fn from_u32(width: u32, height: u32) -> Self {
        Size {
            width: width as f32,
            height: height as f32,
        }
    }

    ///  aspect ratio
    pub fn ratio(&self) -> f32 {
        self.width / self.height
    }

    /// Adjust `self.width` according to `new.width`.
    pub fn resize_by_width(&self, new_width: f32) -> Self {
        let r = self.ratio();
        let w = new_width;
        let h = w / r;

        // e.g. w = 3, h = 4
        //      w = (w/2)*2 = 2
        //      h = (h/2)*2 = 4
        let h = (h * 2.0) / 2.0;

        Size::new(w, h)
    }

    pub fn resize_by_height(&self, new_height: f32) -> Self {
        let r = self.ratio();
        let h = new_height;
        let w = r * h;

        let w = (w / 2.0) * 2.0;
        let h = (h / 2.0) * 2.0;

        Size::new(w, h)
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn size(&self) -> f32 {
        self.width * self.height
    }

    pub fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn is_zero(&self) -> bool {
        self.len() == 0
    }
}

impl MetaSize {
    pub fn new(canvas: Size, window: Size) -> Self {
        Self { window, canvas }
    }
}

impl Mul<f32> for Size {
    type Output = Size;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.width() * rhs, self.height() * rhs)
    }
}
