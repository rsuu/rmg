pub type SelfResult<T> = Result<T, MyError>;
pub type MyResult = SelfResult<()>;

// tags
pub type Names = Vec<String>;

#[derive(Debug)]
pub enum MyError {
    ErrCanvas(Box<dyn std::error::Error>),               // canvas
    ErrEvent(String),                                    // event
    ErrIo(std::io::Error),                               // io
    ErrFromUtf8(std::string::FromUtf8Error),             // utf8
    ErrImage(image::error::ImageError),                  // image
    ErrBufferImage(fast_image_resize::ImageBufferError), // resize image
    ErrDifferentTypesOfPixels(fast_image_resize::DifferentTypesOfPixelsError), // resize imgae
    ErrMiniserde(miniserde::Error),                      //
    ErrSpeedy(speedy::Error),                            // metadata
    ErrLexopt(lexopt::Error),                            // args
    ErrLexoptStr(std::ffi::OsString),
    ErrParseInt(std::num::ParseIntError),
    ErrTryFromSlice(std::array::TryFromSliceError),
    ErrMulDivImage(fast_image_resize::MulDivImageError),
    ErrNull(()),
    ErrMagickNumber,
}

impl From<String> for MyError {
    fn from(e: String) -> Self {
        MyError::ErrEvent(e)
    }
}

impl From<()> for MyError {
    fn from(e: ()) -> Self {
        MyError::ErrNull(e)
    }
}

impl From<std::array::TryFromSliceError> for MyError {
    fn from(e: std::array::TryFromSliceError) -> Self {
        MyError::ErrTryFromSlice(e)
    }
}

impl From<fast_image_resize::MulDivImageError> for MyError {
    fn from(e: fast_image_resize::MulDivImageError) -> Self {
        MyError::ErrMulDivImage(e)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(e: std::num::ParseIntError) -> Self {
        MyError::ErrParseInt(e)
    }
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::ErrIo(e)
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        MyError::ErrFromUtf8(e)
    }
}

impl From<std::ffi::OsString> for MyError {
    fn from(e: std::ffi::OsString) -> Self {
        MyError::ErrLexoptStr(e)
    }
}

impl From<lexopt::Error> for MyError {
    fn from(e: lexopt::Error) -> Self {
        MyError::ErrLexopt(e)
    }
}

impl From<speedy::Error> for MyError {
    fn from(e: speedy::Error) -> Self {
        MyError::ErrSpeedy(e)
    }
}

impl From<miniserde::Error> for MyError {
    fn from(e: miniserde::Error) -> Self {
        MyError::ErrMiniserde(e)
    }
}

impl From<fast_image_resize::ImageBufferError> for MyError {
    fn from(e: fast_image_resize::ImageBufferError) -> Self {
        MyError::ErrBufferImage(e)
    }
}

impl From<image::error::ImageError> for MyError {
    fn from(e: image::error::ImageError) -> Self {
        MyError::ErrImage(e)
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for MyError {
    fn from(e: std::boxed::Box<dyn std::error::Error>) -> Self {
        MyError::ErrCanvas(e)
    }
}

impl From<fast_image_resize::DifferentTypesOfPixelsError> for MyError {
    fn from(e: fast_image_resize::DifferentTypesOfPixelsError) -> Self {
        MyError::ErrDifferentTypesOfPixels(e)
    }
}
