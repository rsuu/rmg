// TODO:
//
// - [ ] feat: {turn,jump,mark} page
// - [ ] feat: history
// - [ ] feat: scroll {double page, up}
// - [ ] feat: fullscreen
// - [ ] ?feat: padding L/R VS resize BG and FG

// ingore main.rs
#[cfg(feature = "web")]
mod web;

// ==========================================
pub mod archive;
pub mod config;
pub mod img;
pub mod render;

// ==========================================
pub use std::{
    mem,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread,
};

// trait
pub use {archive::ForExtract, img::TMetaSize, render::ForAsyncTask};
// struct/enum
pub use {
    archive::{ArchiveType, FileInfo, FileList},
    config::rsconf::Config,
    img::{FilterType, MetaSize, Size, TransRgb, TransRgba},
    render::{
        keymap::{self, KeyMap, Map},
        once::Once,
        scroll::Scroll,
        turn::Turn,
        window::{Canvas, WindowPosition},
        AsyncTask, Buffer, Data, Img, ImgFormat, Page, PageList, ReaderMode, TaskResize, ViewMode,
    },
};
// fn
pub use {config::rsconf::print_help, render::display::cat_img, render::keymap::match_event};

// ==========================================
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SLEEP_MS: u64 = 1000 / 120;
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
pub fn sleep_ms(ms: u64) {
    use std::time::Duration;

    std::thread::sleep(Duration::from_millis(ms));
}

pub fn sleep() {
    sleep_ms(SLEEP_MS);
}
