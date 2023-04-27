pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SLEEP_MS: u32 = 1000 / 120;
pub const FPS: u32 = 1000 / 25;
pub const EXT_LIST: &[&str] = &[
    "jpg", "jpeg", //
    "png",  //
    "heic", "heif", //
    "avif", //
    "ase", "aseprite", //
    "gif",      //
    "svg",
];

// ==========================================
pub mod archive;
pub mod config;
pub mod img;
pub mod render;

// ==========================================
pub use fir::FilterType;
pub use std::{
    mem,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread::{self, sleep_ms, yield_now},
};
pub use {
    archive::{ArchiveType, FileInfo, FileList, ForExtract},
    config::rsconf::{print_help, Config},
    img::{MetaSize, Size, TMetaSize, TransRgb, TransRgba},
    render::{
        display::cat_img,
        keymap::{self, match_event, KeyMap, Map},
        once::Once,
        scroll::Scroll,
        turn::Turn,
        window::Canvas,
        AsyncTask, Buffer, Data, ForAsyncTask, Img, ImgFormat, Page, PageList, ReaderMode,
        TaskResize, ViewMode,
    },
};

// ==========================================
pub fn sleep() {
    std::thread::sleep_ms(SLEEP_MS);
}
