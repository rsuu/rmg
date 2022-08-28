use crate::{
    color::format,
    config::rsconf::Config,
    img::size::Size,
    utils::{
        err::Res,
        types::{MyError, SelfResult},
    },
};
use dirs_next;
use emeta::meta;
use lexopt::{self, prelude::*};
use std::{fs::File, io::Write, path::PathBuf, process::exit};

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

    pub fn parse(&mut self) -> SelfResult<()> {
        let mut parser = lexopt::Parser::from_env();

        while let Some(arg) = parser.next()? {
            match arg {
                Long("help") | Short('h') => {
                    print_help();
                }

                Long("config") | Short('c') => {
                    self.config_path = Some(parser.value()?.into_string()?);
                }

                Long("size") | Short('s') => {
                    let size = parser.value()?.into_string()?;
                    let size = size.as_str().split(',').collect::<Vec<&str>>();

                    if let Ok(w) = size[0].parse::<usize>() {
                        if let Ok(h) = size[1].parse::<usize>() {
                            self.size = Some(Size::new(w, h));
                        }
                    }
                }

                Long("rename") => {
                    let is_rename = parser.value()?.into_string()?;

                    if is_rename.as_str() == "false" {
                        self.rename = false
                    } else {
                        panic!("")
                    };
                }

                Long("pad") => {
                    let pad = parser.value()?.into_string()?;
                    self.rename_pad = pad.parse::<usize>()?;
                }

                Long("format") => {
                    let format = parser.value()?.into_string()?;
                    self.format = match format.as_str() {
                        "rgb8" => Some(format::PixelFormat::Rgb8),
                        "rgba8" => Some(format::PixelFormat::Rgba8),
                        _ => None,
                    };
                }

                Short('m') | Long("meta") => {
                    let sub = parser.value()?.into_string()?;

                    match sub.to_ascii_lowercase().as_str() {
                        "d" | "display" => {
                            self.meta_display = true;
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

                Value(v) => self.file_path = Some(v.into_string()?),

                _ => {
                    print_help();
                }
            }
        }

        Ok(())
    }

    pub fn set_size(&mut self, config: &mut Config) {
        if let Some(size) = self.size {
            config.base.size = size;
        } else {
            // default
        };
    }

    pub fn set_config_path(&mut self) {
        if self.config_path.is_none() {
            let mut config_path = PathBuf::new();

            if let Some(path) = dirs_next::config_dir() {
                config_path.push(path.as_path());

                if config_path.as_path().is_dir() {
                } else {
                    std::fs::create_dir(config_path.as_path()).unwrap();
                }

                config_path.push("rmg/config.rs");

                if config_path.as_path().is_file() {
                } else {
                    let mut f = File::create(config_path.as_path()).unwrap();

                    f.write_all(include_bytes!("../config/default_config.rs"))
                        .unwrap();
                }

                self.config_path = Some(config_path.to_str().unwrap().to_string());

                log::debug!("config_path == {:?}", config_path.as_path());
            } else {
            }
        } else {
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

fn print_help() -> ! {
    eprintln!(
        r#"rmg: 0.0.11
Manga Reader

USAGE:
    rmg [OPTIONS] file

OPTIONS:
    -h, --help       Prints help information
    -V, --version    Prints version information

    -s, --size       Max width and height of buffer
                     e.g. rmg --size 900,900
    -c, --config     ...
    -m, --meta       ...

    --pad            ...
    --rename         ...
    --format         ...
"#
    );
    exit(127);
}
