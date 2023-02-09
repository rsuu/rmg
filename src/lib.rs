#![feature(async_fn_in_trait)]

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const FPS: u64 = 1000 / 25;
pub const EXT_LIST: &[&str] = &[
    "jpg", "jpeg", "png", "heic", "heif", "avif", "ase", "aseprite", "gif", "svg",
];

// block | expr | ident | item | lifetime | literal
// meta | pat | pat_param | path | stmt | tt | ty | vis
//
// #[macro_export]
// macro_rules! check {
//    ( $( $args:expr ),* ) => {
//        #[cfg(debug_assertions)]
//        {
//            dbg!( $( $args ),* );
//        }
//
//        #[cfg(not(debug_assertions))]
//        { }
//    }
// }

// #[macro_export]
// macro_rules! error_from {
//     ( $($l:path, $r:path ;)* ) => {
//         $(
//             impl From<$l> for MyErr {
//                 fn from(e: $l) -> Self {
//                     $r(e)
//                 }
//             }
//         )*
//     }
// }

/////////////////////////////////////////////////
pub fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}
/////////////////////////////////////////////////
pub mod utils {
    pub mod traits;
}

pub mod archive {
    pub mod utils;

    pub mod dir;
    pub mod file;

    // feature
    pub mod tar;
    pub mod zip;
}

pub mod img {
    pub mod covert;
    pub mod resize;
    pub mod size;

    // feature
    pub mod ase;
    pub mod gif;
    pub mod heic;
    pub mod svg;
}

pub mod color {
    pub mod format;
    pub mod rgb;
    pub mod rgba;
}

pub mod render {
    pub mod display;
    pub mod keymap;
    pub mod view;
    pub mod window;

    pub mod once;
    pub mod scroll;
    pub mod turn;
}

pub mod config {
    pub mod history;
    pub mod rsconf;
}
