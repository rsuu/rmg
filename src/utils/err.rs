pub type Res<T> = Result<T, MyErr>;

#[derive(Debug)]
pub enum MyErr {
    Canvas(Box<dyn std::error::Error>),
    Event(String),
    Io(std::io::Error),
    LexoptStr(std::ffi::OsString),
    ParseInt(std::num::ParseIntError),
    TryFromSlice(std::array::TryFromSliceError),
    FromUtf8(std::string::FromUtf8Error),
    Image(image::error::ImageError),
    BufferImage(fir::ImageBufferError),
    MulDivImage(fir::MulDivImageError),
    DifferentTypesOfPixels(fir::DifferentTypesOfPixelsError),
    Lexopt(lexopt::Error),
    WalkDir(walkdir::Error),

    Null,

    Todo,

    FeatHeic,
    FeatSvg,
    FeatAse,
    FeatTar,
    FeatZip,
}

impl From<()> for MyErr {
    fn from(_e: ()) -> Self {
        MyErr::Null
    }
}

crate::error_from! {
    fir::DifferentTypesOfPixelsError
      , MyErr::DifferentTypesOfPixels;
    fir::ImageBufferError
      , MyErr::BufferImage;
    fir::MulDivImageError
      , MyErr::MulDivImage;
    image::error::ImageError
      , MyErr::Image;
    lexopt::Error
      , MyErr::Lexopt;
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
