#[macro_use]
pub mod utils {
    pub mod types;
    #[macro_use]
    pub mod macros;
}

pub mod archive {
    pub mod tar;
    pub mod zip;
}

pub mod files {
    pub mod list;
}

pub mod math {
    pub mod arrmatrix;
}

pub mod img {
    pub mod size;
}

pub mod color {
    pub mod format;
    pub mod rgb;
    pub mod rgba;
}

pub mod reader {
    pub mod canvas;
    pub mod display;
}

pub mod cli {
    pub mod parse;
}

pub mod metadata {
    pub mod meta;
    pub mod tags;
}
