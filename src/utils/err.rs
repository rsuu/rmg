pub type Res<T> = Result<T, MyErr>;

#[derive(Debug)]
pub enum MyErr {
    Canvas(Box<dyn std::error::Error>), // canvas
    Event(String),                      // event
    Io(std::io::Error),                 // io
    LexoptStr(std::ffi::OsString),
    ParseInt(std::num::ParseIntError),
    TryFromSlice(std::array::TryFromSliceError),
    FromUtf8(std::string::FromUtf8Error), // utf8

    Image(image::error::ImageError), // image

    BufferImage(fast_image_resize::ImageBufferError), // resize image
    MulDivImage(fast_image_resize::MulDivImageError),
    DifferentTypesOfPixels(fast_image_resize::DifferentTypesOfPixelsError), // resize imgae

    //   Miniserde(miniserde::Error), //

    // Speedy(speedy::Error), // metadata
    Lexopt(lexopt::Error), // args
    WalkDir(walkdir::Error),

    Null(()),
    MagickNumber,
}

impl From<()> for MyErr {
    fn from(e: ()) -> Self {
        MyErr::Null(e)
    }
}

crate::error_from! {
    fast_image_resize::DifferentTypesOfPixelsError
      , MyErr::DifferentTypesOfPixels;
    fast_image_resize::ImageBufferError
      , MyErr::BufferImage;
    fast_image_resize::MulDivImageError
      , MyErr::MulDivImage;
    image::error::ImageError
      , MyErr::Image;
    lexopt::Error
      , MyErr::Lexopt;
    //miniserde::Error
    //  , MyErr::Miniserde;
    //speedy::Error
    //  , MyErr::Speedy;
    std::array::TryFromSliceError
      , MyErr::TryFromSlice;
    std::boxed::Box<dyn std::error::Error>
      , MyErr::Canvas;
    std::ffi::OsString
      , MyErr::LexoptStr;
    std::io::Error
      , MyErr::Io;
    std::num::ParseIntError
      , MyErr::ParseInt;
    std::string::FromUtf8Error
      , MyErr::FromUtf8;
    std::string::String
      , MyErr::Event;
    walkdir::Error
      , MyErr::WalkDir;
}
