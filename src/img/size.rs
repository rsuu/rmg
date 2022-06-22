#[derive(Debug, Clone, Copy)]
pub struct MetaSize<T> {
    pub screen: Size<T>,
    pub window: Size<T>,
    pub image: Size<T>,
    pub fix: Size<T>,
}

pub trait TMetaSize {
    type T;

    fn new(sw: Self::T, sh: Self::T, ww: Self::T, wh: Self::T, iw: Self::T, ih: Self::T) -> Self;

    fn resize(&mut self);
}

impl TMetaSize for MetaSize<u32> {
    type T = u32;

    fn new(sw: Self::T, sh: Self::T, ww: Self::T, wh: Self::T, iw: Self::T, ih: Self::T) -> Self {
        MetaSize {
            screen: Size::<u32>::new(sw, sh),
            window: Size::<u32>::new(ww, wh),
            image: Size::<u32>::new(iw, ih),
            fix: Size::<u32>::new(0, 0),
        }
    }

    fn resize(&mut self) {
        // !!! important
        // if width = 3 height = 4
        // do width = (width/2) * 2    = (3/2) * 2 = 1 * 2 = 2
        //    height = (height/2) * 2  = (4/2) * 2 = 2 * 2 = 4
        // We will get a bug if miss it
        // (╯°Д°)╯︵ ┻━┻
        // TODO
        let w = self.window.width as f32;
        let q = self.image.width as f32 / self.image.height as f32;
        let h = w / q;

        self.fix.width = (w as Self::T / 2) * 2;
        self.fix.height = (h as Self::T / 2) * 2;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Size { width, height }
    }
}
