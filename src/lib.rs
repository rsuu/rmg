pub use cfg_if::cfg_if;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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

pub mod utils {
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

    pub mod err;
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

pub mod files {
    pub mod file;
    pub mod list;
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
    pub mod buffer;
    pub mod display;
    pub mod keymap;
    pub mod window;
}

pub mod cli {
    pub mod parse;
}

pub mod config {
    pub mod rsconf;
}
