pub mod utils {
    pub mod err;
    pub mod macros;
    pub mod types;
}

pub mod archive {
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
