use crate::{
    color::format,
    config::rsconf::Config,
    img::size::Size,
    utils::err::{MyErr, Res},
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

    pub fn parse(&mut self) -> Res<()> {
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

    pub fn set_config_path(&self) -> Option<Config> {
        if let Some(config_file) = &self.config_path {
            // e.g. rmg --config config.rs
            return Some(Config::parse_from(config_file.as_str()));
        } else {
            let mut config_file = PathBuf::new();

            // e.g. ~/.config/rmg/config.rs
            if let Some(path) = dirs_next::config_dir() {
                if path.as_path().is_dir() {
                    config_file.push(path.as_path());
                    config_file.push("rmg/config.rs");

                    if config_file.as_path().is_file() {
                        return Some(Config::parse_from(config_file.as_path()));
                    } else {
                        // doing nothing
                    }
                } else {
                    std::fs::create_dir(path.as_path()).unwrap();
                }
            } else {
                // e.g. Config::default()
            }
        }

        None
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

fn print_help() -> ! {
    use crate::VERSION;

    println!(
        r#"rmg: {version}
Manga Reader

USAGE:
    rmg [OPTIONS] file

OPTIONS:
    -h, --help       Prints help information
    -V, --version    Prints version information

    -s, --size       Max width and height of the buffer
                     e.g. rmg --size 900,900
    -c, --config     Reset the config path
    -m, --meta       ...

    --rename         ...
"#,
        version = VERSION
    );
    exit(127);
}
