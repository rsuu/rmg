use crate::img::size::Size;
use lexopt;
use lexopt::prelude::*;
use std::process::exit;

#[derive(Debug)]
pub struct Args {
    pub sub: SubCommand,
    pub file_path: Option<String>,
    pub dir_path: Option<String>,

    pub size: Option<Size<u32>>,
}

impl Args {
    pub fn new() -> Self {
        Args {
            sub: SubCommand::Help,
            file_path: None,
            dir_path: None,
            size: None,
        }
    }

    pub fn get_args() -> Result<Self, lexopt::Error> {
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
                    if let Ok(w) = size[0].parse::<u32>() {
                        if let Ok(h) = size[1].parse::<u32>() {
                            args.size = Some(Size::new(w, h));
                        }
                    }
                }

                Value(v) => args.file_path = Some(v.into_string()?),

                _ => return Err(arg.unexpected()),
            }
        }

        Ok(args)
    }

    pub fn parse(&self) {}
}

fn print_help() -> ! {
    eprintln!("help");
    exit(127);
}

#[derive(Debug)]
pub enum SubCommand {
    Help,
    Archive,
}
