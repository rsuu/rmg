pub mod app;
pub mod archive;
pub mod data;
pub mod frame;
pub mod shape;
pub mod ui;
pub mod utils;

pub use {
    app::{
        buffer::*, canvas::*, draw::*, gesture::*, layout::*, page::*, state::*, task::*,
        window::*, *,
    },
    archive::*,
    data::{config::*, *},
    frame::*,
    shape::{circle::*, rect::*},
    ui::{elem::*, style::*, *},
    utils::{affine::*, filter::*, size::*, vec2::*, *},
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
pub const SUPPORTED_FORMAT: &[&str] = &[
    "jpg", "jpeg", //
    "png",  //
    "heic", "heif", //
    "avif", //
    "ase", "aseprite", //
    "gif",      //
    "svg",
];
