pub type SelfResult<T> = Result<T, MyError>;
pub type MyResult = SelfResult<()>;

// tags
pub type Names = Vec<String>;

#[derive(Debug)]
pub enum MyError {
    ErrCanvas(Box<dyn std::error::Error>), // canvas
    ErrEvent(String),                      // event
    ErrIo(std::io::Error),                 // io
    ErrLexoptStr(std::ffi::OsString),
    ErrParseInt(std::num::ParseIntError),
    ErrTryFromSlice(std::array::TryFromSliceError),
    ErrFromUtf8(std::string::FromUtf8Error), // utf8

    ErrImage(image::error::ImageError), // image

    ErrBufferImage(fast_image_resize::ImageBufferError), // resize image
    ErrMulDivImage(fast_image_resize::MulDivImageError),
    ErrDifferentTypesOfPixels(fast_image_resize::DifferentTypesOfPixelsError), // resize imgae

    ErrMiniserde(miniserde::Error), //

    ErrSpeedy(speedy::Error), // metadata

    ErrLexopt(lexopt::Error), // args

    ErrIntegerOrSdl(sdl2::IntegerOrSdlError),
    ErrTextureValue(sdl2::render::TextureValueError),
    ErrWindowBuild(sdl2::video::WindowBuildError),
    ErrFontError(sdl2::ttf::FontError),

    ErrNull(()),
    ErrMagickNumber,
}

impl From<()> for MyError {
    fn from(e: ()) -> Self {
        MyError::ErrNull(e)
    }
}

crate::impl_from! {
    fast_image_resize::DifferentTypesOfPixelsError
      , MyError::ErrDifferentTypesOfPixels;
    fast_image_resize::ImageBufferError
      , MyError::ErrBufferImage;
    fast_image_resize::MulDivImageError
      , MyError::ErrMulDivImage;
    image::error::ImageError
      , MyError::ErrImage;
    lexopt::Error
      , MyError::ErrLexopt;
    miniserde::Error
      , MyError::ErrMiniserde;
    sdl2::IntegerOrSdlError
      , MyError::ErrIntegerOrSdl;
    sdl2::render::TextureValueError
      , MyError::ErrTextureValue;
    sdl2::ttf::FontError
      , MyError::ErrFontError;
    sdl2::video::WindowBuildError
      , MyError::ErrWindowBuild;
    speedy::Error
      , MyError::ErrSpeedy;
    std::array::TryFromSliceError
      , MyError::ErrTryFromSlice;
    std::boxed::Box<dyn std::error::Error>
      , MyError::ErrCanvas;
    std::ffi::OsString
      , MyError::ErrLexoptStr;
    std::io::Error
      , MyError::ErrIo;
    std::num::ParseIntError
      , MyError::ErrParseInt;
    std::string::FromUtf8Error
      , MyError::ErrFromUtf8;
    std::string::String
      , MyError::ErrEvent;
}
