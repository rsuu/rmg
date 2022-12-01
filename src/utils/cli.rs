use crate::{config::rsconf::Config, img::size::Size, utils::err::Res};
use dirs_next;
use lexopt::{self, prelude::*};
use std::{path::PathBuf, process::exit};

#[derive(Debug)]
pub struct Args {
    pub config_path: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        Args { config_path: None }
    }

    pub fn parse(&mut self, config: &mut Config) -> Res<()> {
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

                    let (w, h) = (
                        size[0].parse::<usize>().unwrap_or_default(),
                        size[1].parse::<usize>().unwrap_or_default(),
                    );

                    config.base.size = Size::new(w, h);
                }

                Long("pad") => {
                    let pad = parser.value()?.into_string()?;
                    config.base.rename_pad = pad.parse::<u8>()?;
                }

                Value(v) => {
                    config.cli.file_path = Some(v.into_string()?);
                }

                _ => {
                    print_help();
                }
            }
        }

        Ok(())
    }

    pub fn init_config(&self) -> Config {
        // parse from input
        let mut res = Config::default();

        if let Some(config_file) = &self.config_path {
            // e.g. rmg --config config.rs
            res = Config::parse_from(config_file.as_str());
        } else {
            // parse from file
            let mut config_path = PathBuf::new();

            // e.g. ~/.config/rmg/config.rs
            if let Some(path) = dirs_next::config_dir() {
                if path.as_path().is_dir() {
                    config_path.push(path.as_path());
                    config_path.push("rmg/config.rs");

                    log::debug!("config_path: {:?}", config_path);

                    if config_path.as_path().is_file() {
                        res = Config::parse_from(config_path.as_path());
                    } else {
                        // doing nothing
                    }
                } else {
                    // doing nothing
                }
            } else {
                // default
            }
        };

        res
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

    --pad            ...
"#,
        version = VERSION
    );
    exit(127);
}
