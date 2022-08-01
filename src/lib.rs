pub mod utils {
    pub mod types;

    pub mod macros;
}

pub mod archive {
    #[cfg(feature = "ex_tar")]
    pub mod tar;

    #[cfg(feature = "ex_zip")]
    pub mod zip;

    //#[cfg(feature = "ex_zstd")]
    //pub mod zstd;
}

pub mod files {
    pub mod dir;
    pub mod file;
    pub mod list;
}

pub mod math {
    pub mod arrmatrix;
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
    //pub mod canvas;
    pub mod display;
    pub mod keymap;
    pub mod mini;
}

pub mod cli {
    pub mod parse;
}

pub mod config {
    pub mod rsconf;
}
