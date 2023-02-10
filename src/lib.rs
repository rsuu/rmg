#![feature(async_fn_in_trait)]

// ==========================================
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const FPS: u32 = 1000 / 25;
pub const EXT_LIST: &[&str] = &[
    "jpg", "jpeg", "png", "heic", "heif", "avif", "ase", "aseprite", "gif", "svg",
];

// ==========================================
pub mod utils {
    // ==========================================
    impl<AnyType: ?Sized> AutoTrait for AnyType {}

    pub trait AutoTrait {
        // usage:
        //     <u8>::_size()
        //     <Option<u32>>::_size()
        fn _size() -> usize
        where
            Self: Sized,
        {
            std::mem::size_of::<Self>()
        }
    }
}

// ==========================================
pub mod archive {
    pub mod utils;

    pub mod dir;
    pub mod file;

    // feature
    pub mod tar;
    pub mod zip;
}

pub mod img {
    pub mod utils;

    // feature
    pub mod ase;
    pub mod gif;
    pub mod heic;
    pub mod svg;
}

pub mod render {
    pub mod utils;

    pub mod display;
    pub mod keymap;
    pub mod window;

    pub mod once;
    pub mod scroll;
    pub mod turn;
}

pub mod config {
    pub mod history;
    pub mod rsconf;
}
