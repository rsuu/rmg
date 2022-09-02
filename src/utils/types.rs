// tags
pub type Names = Vec<String>;

#[derive(Debug, Copy, Clone)]
pub enum ArchiveType {
    Tar,
    Zip,
    Dir,
}
