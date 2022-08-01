use crate::{
    color::format,
    img::size::Size,
    utils::types::{MyError, SelfResult},
};
use emeta::meta;
use lexopt::{self, prelude::*};
use std::process::exit;

#[derive(Debug)]
pub struct Args {
    pub config_path: Option<String>,

    pub file_path: Option<String>,
    pub dir_path: Option<String>,

    pub size: Option<Size<usize>>,

    pub rename: bool,
    pub rename_pad: usize,

    pub format: Option<format::PixelFormat>,

    pub meta_display: bool,
}

impl Args {
    pub fn new() -> Self {
        Args {
            config_path: None,

            file_path: None,
            dir_path: None,

            size: None,

            rename: true,
            rename_pad: 6,

            format: None,

            meta_display: false,
        }
    }

    pub fn get_args() -> SelfResult<Self> {
        let mut args = Args::new();
        let mut parser = lexopt::Parser::from_env();

        while let Some(arg) = parser.next()? {
            match arg {
                Long("help") | Short('h') => {
                    print_help();
                }

                Long("config") | Short('c') => {
                    if args.config_path.is_none() {
                        args.config_path = Some(parser.value()?.into_string()?);
                    } else {
                    }
                }

                Long("size") | Short('s') => {
                    let size = parser.value()?.into_string()?;
                    let size = size.as_str().split(',').collect::<Vec<&str>>();

                    if let Ok(w) = size[0].parse::<usize>() {
                        if let Ok(h) = size[1].parse::<usize>() {
                            args.size = Some(Size::new(w, h));
                        }
                    }
                }

                Long("rename") => {
                    let is_rename = parser.value()?.into_string()?;

                    if is_rename.as_str() == "false" {
                        args.rename = false
                    } else {
                        panic!("")
                    };
                }

                Long("pad") => {
                    let pad = parser.value()?.into_string()?;
                    args.rename_pad = pad.parse::<usize>()?;
                }

                Long("format") => {
                    let format = parser.value()?.into_string()?;
                    args.format = match format.as_str() {
                        "rgb8" => Some(format::PixelFormat::Rgb8),
                        "rgba8" => Some(format::PixelFormat::Rgba8),
                        _ => None,
                    };
                }

                Short('m') | Long("meta") => {
                    let sub = parser.value()?.into_string()?;

                    match sub.to_ascii_lowercase().as_str() {
                        "d" | "display" => {
                            args.meta_display = true;
                        }

                        "f" | "from" => {
                            let file_path = parser.value()?.into_string()?;

                            // echo xxx | rmg -m f -
                            match file_path.as_str() {
                                "-" => {
                                    let meta = meta::MetaData::from_pipe().unwrap();
                                    meta.display();
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                Value(v) => args.file_path = Some(v.into_string()?),

                _ => return Err(MyError::ErrLexopt(arg.unexpected())),
            }
        }

        Ok(args)
    }

    pub fn parse(&self) {}
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

fn print_help() -> ! {
    eprintln!(
        r#"rmg 0.0.8
Manga Reader

USAGE:
    rmg [OPTIONS] path

META OPTIONS:
    -h, --help       Prints help information
    -V, --version    Prints version information

DISPLAY OPTIONS:
    -s, --size       ...
    -c, --config     ...
    -m, --meta       ...

OTHER OPTIONS:
    --pad            ...
    --rename         ...
    --format         ...
"#
    );
    exit(127);
}
