pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const EXT_LIST: &[&str] = &[
    "jpg", "jpeg", "png", "heic", "heif", "avif", "ase", "aseprite",
];
pub static mut TIMER: usize = 0;

#[inline]
pub fn has_supported(path: &str) -> bool {
    // is dir
    if path.ends_with('/') {
        return false;
    }

    for ext in EXT_LIST {
        // e.g. ".jpg"
        if path.ends_with(format!(".{}", ext).as_str()) {
            return true;
        } else {
        }
    }

    false
}

// block | expr | ident | item | lifetime | literal
// meta | pat | pat_param | path | stmt | tt | ty | vis
// #[macro_export]
// macro_rules! unwrap_or_return {
//     ( $e:expr , $err:expr) => {
//         match $e {
//             Ok(x) => x,
//             Err(e) => return Err($err(e)),
//         }
//     };
// }
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

#[macro_export]
macro_rules! error_from {
    ( $($l:path, $r:path ;)* ) => {
        $(
            impl From<$l> for MyErr {
                fn from(e: $l) -> Self {
                    $r(e)
                }
            }
        )*
    }
}

pub mod utils {
    pub mod cli;
    pub mod err;
    pub mod file;
    pub mod traits;
}

pub mod archive {

    #[derive(Debug, Copy, Clone)]
    pub enum ArchiveType {
        Tar,
        Zip,
        Dir,
    }

    pub mod dir;

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
}

pub mod color {
    pub mod format;
    pub mod rgb;
    pub mod rgba;
}

pub mod reader {
    pub mod display;
    pub mod keymap;
    pub mod view;
    pub mod window;

    pub mod scroll;
    pub mod turn;
}

pub mod config {
    pub mod history;
    pub mod rsconf;
}
