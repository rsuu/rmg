pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const FPS: u32 = 1000 / 25;
pub const EXT_LIST: &[&str] = &[
    "jpg", "jpeg", "png", "heic", "heif", "avif", "ase", "aseprite", "gif", "svg",
];

// ==========================================
pub mod archive;
pub mod config;
pub mod img;
pub mod render;
