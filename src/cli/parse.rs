use crate::{
    color::format,
    img::size::Size,
    utils::types::{MyError, SelfResult},
};
use emeta::lib::{meta, tags};
use lexopt::{self, prelude::*};
use std::process::exit;

#[derive(Debug)]
pub struct Args {
    pub file_path: Option<String>,
    pub dir_path: Option<String>,

    pub size: Option<Size<usize>>,

    pub rename: bool,
    pub rename_pad: usize,

    pub format: Option<format::PixelFormat>,
}

impl Args {
    pub fn new() -> Self {
        Args {
            file_path: None,
            dir_path: None,

            size: None,

            rename: true,
            rename_pad: 6,

            format: None,
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
                            let mut meta = meta::MetaData::new();

                            meta.male = Some(tags::TagMale {
                                name: vec!["www".to_string()],
                            });

                            meta.write_to_file("/root/t/rmg").unwrap();
                            meta.read_from_file("/root/t/rmg").unwrap();

                            meta.display();
                            meta.to_json();
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
    eprintln!("help");
    exit(127);
}
