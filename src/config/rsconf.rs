// TODO: rewrite

use crate::{img::*, render::ViewMode, WindowPosition, VERSION};
use dirs_next;
use esyn::*;
use fir;
use lexopt::{self, prelude::*};
use pico_args::Arguments;
use std::{fs::File, io::Read, path::Path, path::PathBuf, process::exit};

#[derive(Debug, EsynDe)]
pub struct Config {
    pub base: Base,
    pub keymap: Keymap<char>,
    pub window: Window,
    pub cli: Cli,
}

#[derive(Debug, EsynDe)]
pub struct Cli {
    pub config_path: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, EsynDe)]
pub struct Base {
    pub size: Size<u32>,      // window size
    pub font: Option<String>, // font file
    pub rename_pad: u8,       // pad (default: 0)
    pub invert_mouse: bool,   // (default: false)
    pub filter: FilterType,
    pub step: u8,
    pub view_mode: ViewMode,
    pub limit: u8, // page
    pub thread_limit: u8,
    //pub pos_start: (u32, u32),
}

#[derive(Debug, EsynDe)]
pub struct Window {
    pub borderless: bool,
    pub topmost: bool,
    pub resize: bool,
    pub none: bool,
    pub postition: WindowPosition,
}

// rewrite
#[derive(Debug, EsynDe, Copy, Clone)]
pub struct Keymap<Char_> {
    pub up: Char_,    // page up
    pub down: Char_,  // page down
    pub left: Char_,  // move to left
    pub right: Char_, // move to right
    pub exit: Char_,  // exit
    pub fullscreen: Char_,
}

#[derive(Debug, EsynDe)]
pub enum ConfigType {
    Base,
    Keymap,
    Window,
}

pub fn print_help() -> ! {
    // TODO:
    println!(
        r#"
rmg {VERSION}

Tiny And Fast Manga/Image Viewer

USAGE:
    rmg [OPTIONS] [FLAGS] <path>

ARGS:
    <path>  A file or directory.

FLAGS:
    --invert-mouse                  [default: false]
            e.g. `rmg --invert-mouse true`
    --window-borderless             [default: false]
            Display borderless
    --window-resize                 [default: false]
            Resize window
    --window-topmost                [default: false]
            Set as topmost
    --window-none                   [default: true]
            Set as transparency
    --window-position
            Sets the position of the window
            e.g. `rmg --window-position 500,0`


OPTIONS:
    -h, --help
            Prints help information.
    -s, --size
            Reset the width and the height of the buffer.
            e.g. `rmg --size 900x900`
    -c, --config
            Specify the config path
    -p, --rename-pad
            Padding 0 in filename
    -m, --mode
            (TODO)
"#,
    );

    exit(0);
}

impl Config {
    pub fn new() -> Config {
        Config {
            base: Base::default(),
            keymap: Keymap::default(),
            cli: Cli {
                config_path: None,
                file_path: None,
            },
            window: Window::default(),
        }
    }

    pub fn try_from_file(&mut self) -> anyhow::Result<()> {
        let mut config_path = PathBuf::new();

        // e.g. ~/.config/rmg/config.rs
        let Some(path) = dirs_next::config_dir() else {
            // skip
            return Ok(());
        };

        if path.as_path().is_dir() {
            config_path.push(path.as_path());
            config_path.push("rmg/config.rs");
        } else if config_path.as_path().is_file() {
        } else {
            // skip
            return Ok(());
        }

        tracing::debug!("config_path: {:?}", config_path);

        let mut f = File::open(&config_path)?;
        let mut code = "".to_string();
        f.read_to_string(&mut code)?;

        let esyn = Esyn::new(&code);
        esyn.init().unwrap();

        let base = EsynBuilder::new()
            .set_fn("base")
            .flag_res()
            .get::<Base>(&esyn)?;

        let keymap = EsynBuilder::new()
            .set_fn("keymap")
            .flag_res()
            .get::<Keymap<char>>(&esyn)?;

        let window = EsynBuilder::new()
            .set_fn("window")
            .flag_res()
            .get::<Window>(&esyn)?;

        self.base = base.get();
        self.keymap = keymap.get();
        self.window = window.get();

        Ok(())
    }

    pub fn try_from_cli(&mut self) -> anyhow::Result<()> {
        let mut args = Arguments::from_env();

        // base
        if let Some(v) = args.opt_value_from_str::<_, String>("--size")? {
            let (w, h) = v.split_once('x').unwrap();
            self.base.size = Size::new(w.parse().unwrap(), h.parse().unwrap());
        }
        if let Some(v) = args.opt_value_from_str::<_, u8>("--pad")? {
            self.base.rename_pad = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, bool>("--invert-mouse")? {
            self.base.invert_mouse = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, String>("--filter")? {
            self.base.filter = FilterType::from_str(&v);
        }
        if let Some(v) = args.opt_value_from_str::<_, String>("--font")? {
            self.base.font = Some(v);
        }
        if let Some(v) = args.opt_value_from_str::<_, u8>("--step")? {
            self.base.step = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, u8>("--limit")? {
            self.base.limit = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, u8>("--thread-limit")? {
            self.base.thread_limit = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, String>("--view-mode")? {
            self.base.view_mode = ViewMode::from_str(&v);
        }

        // window
        if let Some(v) = args.opt_value_from_str::<_, bool>("--window-none")? {
            self.window.none = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, bool>("--window-topmost")? {
            self.window.topmost = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, bool>("--window-borderless")? {
            self.window.borderless = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, bool>("--window-resize")? {
            self.window.resize = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, String>("--position")? {
            let (x, y) = v.split_once(',').unwrap();
            self.window.postition = WindowPosition {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            };
        }

        // ?keymap
        //

        // cli
        if let Some(v) = args.opt_value_from_str::<_, String>("--config-path")? {
            self.cli.config_path = Some(v);
        }
        if let Ok(v) = args.free_from_str() {
            self.cli.file_path = Some(v);
        }

        args.finish();

        Ok(())
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            borderless: false,
            topmost: false,
            resize: false,
            none: true,
            postition: WindowPosition { x: 0, y: 0 },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

impl Default for Keymap<char> {
    fn default() -> Self {
        Keymap {
            up: 'k',
            down: 'j',
            left: 'h',
            right: 'l',
            exit: 'q',
            fullscreen: 'f',
        }
    }
}

impl Default for Base {
    fn default() -> Self {
        let thread_limit = {
            use sysinfo::System;

            let sys = sysinfo::System::new_all();
            sys.physical_core_count().unwrap_or(1) as u8
        };

        Base {
            size: Size::<u32> {
                width: 600,
                height: 600,
            },
            font: None,
            rename_pad: 0,
            invert_mouse: false,
            filter: FilterType::Hamming,
            step: 10,
            view_mode: ViewMode::Scroll,
            limit: 10,
            thread_limit,
        }
    }
}
