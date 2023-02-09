use crate::render::view::{ImgFormat};



impl<AnyType: ?Sized> AutoTrait for AnyType {}

pub trait AutoTrait {
    // usage:
    //     <u8>::_size()
    //     <Option<u32>>::_size()
    fn _size() -> usize
    where
        Self: Sized,
    {
        std::mem::size_of::<Self>()
    }
}

pub trait ExtImageType {
    fn as_fmt(&self) -> ImgFormat;
}
impl ExtImageType for imagesize::ImageType {
    fn as_fmt(&self) -> ImgFormat {
        match *self {
            Self::Aseprite => ImgFormat::Aseprite,
            Self::Gif => ImgFormat::Gif,
            Self::Jpeg => ImgFormat::Jpg,
            Self::Png => ImgFormat::Png,
            Self::Heif => ImgFormat::Heic,

            // FIXME:rmg -t svg xxx.svg
            // format = ImgFormat::Svg;
            _ => ImgFormat::Unknown,
        }
    }
}
