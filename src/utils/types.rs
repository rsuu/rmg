pub type SelfResult<T> = Result<T, MyError>;
pub type MyResult = SelfResult<()>;

// tags
pub type Names = Vec<String>;

#[derive(Debug)]
pub enum MyError {
    ErrCanvas(Box<dyn std::error::Error>), // canvas
    ErrEvent(String),                      // event
    ErrIo(std::io::Error),
    ErrFromUtf8(std::string::FromUtf8Error),
    ErrImage(image::error::ImageError),
    ErrBufferImage(fast_image_resize::ImageBufferError),
    ErrDifferentTypesOfPixels(fast_image_resize::DifferentTypesOfPixelsError),
    ErrMiniserde(miniserde::Error),
    ErrMagickNumber,
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        return MyError::ErrIo(e);
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        return MyError::ErrFromUtf8(e);
    }
}

impl From<String> for MyError {
    fn from(e: String) -> Self {
        return MyError::ErrEvent(e);
    }
}

impl From<miniserde::Error> for MyError {
    fn from(e: miniserde::Error) -> Self {
        return MyError::ErrMiniserde(e);
    }
}

impl From<fast_image_resize::ImageBufferError> for MyError {
    fn from(e: fast_image_resize::ImageBufferError) -> Self {
        return MyError::ErrBufferImage(e);
    }
}

impl From<image::error::ImageError> for MyError {
    fn from(e: image::error::ImageError) -> Self {
        return MyError::ErrImage(e);
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for MyError {
    fn from(e: std::boxed::Box<dyn std::error::Error>) -> Self {
        return MyError::ErrCanvas(e);
    }
}

impl From<fast_image_resize::DifferentTypesOfPixelsError> for MyError {
    fn from(e: fast_image_resize::DifferentTypesOfPixelsError) -> Self {
        return MyError::ErrDifferentTypesOfPixels(e);
    }
}
