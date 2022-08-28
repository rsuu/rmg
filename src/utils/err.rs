use thiserror;

pub type Res<T> = Result<T, MyErr>;

#[derive(Debug, thiserror::Error)]
pub enum MyErr {
    #[error("")]
    WalkDir(#[from] walkdir::Error),

    #[error("")]
    Io(#[from] std::io::Error),

    #[error("")]
    FoundNull,
}
