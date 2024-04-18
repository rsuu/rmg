// use esyn::*;
use esyn::EsynDe;
use fir::{FilterType, ResizeAlg};

impl From<WrapResizeAlg> for ResizeAlg {
    fn from(value: WrapResizeAlg) -> Self {
        match value {
            WrapResizeAlg::Lanczos3 => Self::Convolution(fir::FilterType::Lanczos3),
            WrapResizeAlg::Box => Self::Convolution(FilterType::Box),
            WrapResizeAlg::Hamming => Self::Convolution(FilterType::Hamming),
            WrapResizeAlg::Bilinear => Self::Convolution(FilterType::Bilinear),
            WrapResizeAlg::Mitchell => Self::Convolution(FilterType::Mitchell),
            WrapResizeAlg::CatmullRom => Self::Convolution(FilterType::CatmullRom),

            WrapResizeAlg::Nearest => Self::Nearest,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, EsynDe)]
pub enum WrapResizeAlg {
    #[default]
    Lanczos3,

    Box,
    Bilinear,
    Hamming,
    CatmullRom,
    Mitchell,

    Nearest,
}
