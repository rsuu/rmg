pub mod archive;
pub mod canvas;
pub mod config;
pub mod frame;
pub mod ui;
pub mod utils;
pub mod window;

pub use {
    archive::*,
    canvas::{buffer::*, draw::*, layout::*, page::*, task::*, *},
    config::*,
    frame::*,
    ui::{style::*, tags::*, *},
    utils::{filter::*, size::*, vec2::*, *},
    window::*,
};

pub use std::{
    mem,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};
// ==========================================
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const FPS: f32 = 120.0;
pub const SUPPORTED_FORMAT: &[&str] = &[
    "jpg", "jpeg", //
    "png",  //
    "heic", "heif", //
    "avif", //
    "ase", "aseprite", //
    "gif",      //
    "svg",
];
