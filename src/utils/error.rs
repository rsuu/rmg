pub type MyResult = Result<(), MyError>;

#[derive(Debug)]
pub enum MyError {
    ErrCanvas(Box<dyn std::error::Error>), // canvas
    ErrEvent(String),                      // event
    ErrIo(std::io::Error),
    ErrDecode(image::error::ImageError),
}
