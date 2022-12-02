pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BITMAP_LIST: &[&str] = &["jpg", "png", "heic", "avif"];
pub static mut TIMER: usize = 0;

pub enum OsType {
    Linux,
    Windows,
    Mac,
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

    #[cfg(feature = "ex_tar")]
    pub mod tar;

    #[cfg(feature = "ex_zip")]
    pub mod zip;

    pub mod dir;
}

pub mod img {
    pub mod covert;
    pub mod resize;
    pub mod size;

    #[cfg(feature = "de_heic")]
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
