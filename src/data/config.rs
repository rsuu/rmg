use crate::*;

use esyn::{Esyn, EsynBuilder, EsynDe};
use pico_args::Arguments;
use std::{fs::File, io::Read, process::exit};

const DEFAULT_CONFIG: &str = include_str!("../../assets/config.rs");

#[derive(Debug, Default, Clone, EsynDe)]
pub struct Config {
    pub app: ConfApp,
    pub window: ConfWindow,
    pub canvas: ConfCanvas,
    pub page: ConfPage,
    pub misc: ConfMisc,

    pub gestures: ConfGestures,
    pub layout_double: ConfLayoutDouble,

    pub on_scroll: ConfOnScroll,

    pub once: ConfOnce,
    // TODO: MouseMap
    // ?KeyMap
}

#[derive(Debug, Default, Clone, EsynDe)]
pub struct ConfOnce {
    pub record_gesture_name: Option<String>,
}

#[derive(Debug, Default, Clone, EsynDe)]
pub struct ConfApp {
    pub target: PathBuf,
}

#[derive(Debug, Default, Clone, EsynDe)]
struct ConfWindow {
    pub borderless: bool,
    pub invert_mouse: bool,
}

#[derive(Debug, Default, Clone, EsynDe)]
pub struct ConfCanvas {
    pub layout: Layout,

    pub bg: u32,
    pub cache_limit: u32,

    // unused
    pub font_path: Option<PathBuf>,
}

#[derive(Debug, Default, Clone, EsynDe)]
pub struct ConfPage {
    pub size: Size,
    pub img_resize_algo: WrapResizeAlg,
    pub anim_resize_algo: WrapResizeAlg,
}

#[derive(Debug, Default, Clone, EsynDe)]
pub struct ConfMisc {
    pub padding_filename: u8,
}

#[derive(Debug, Default, Clone, EsynDe)]
pub struct ConfLayoutDouble {
    pub reading_dire: Direction,
}

#[derive(Debug, Default, Clone, EsynDe)]
pub struct ConfGestures {
    // TODO: $XDG_DATA_HOME/rmg/gestures.zip
    pub data_path: String,
    pub min_score: f32,
}

#[derive(Debug, Default, Clone, EsynDe)]
pub struct ConfOnScroll {
    pub step_x: f32,
    pub step_y: f32,
}

impl Config {
    pub fn new() -> eyre::Result<Self> {
        Self::from_str(DEFAULT_CONFIG)
    }

    fn from_str(s: &str) -> eyre::Result<Self> {
        let esyn = Esyn::new(s);
        esyn.init()?;
        // dbg!(&esyn);

        let app = EsynBuilder::new()
            .set_fn("app")
            .flag_res()
            .get::<ConfApp>(&esyn)?
            .get();

        let window = EsynBuilder::new()
            .set_fn("window")
            .flag_res()
            .get::<ConfWindow>(&esyn)?
            .get();

        let canvas = EsynBuilder::new()
            .set_fn("canvas")
            .flag_res()
            .get::<ConfCanvas>(&esyn)?
            .get();

        let page = EsynBuilder::new()
            .set_fn("page")
            .flag_res()
            .get::<ConfPage>(&esyn)?
            .get();

        let misc = EsynBuilder::new()
            .set_fn("misc")
            .flag_res()
            .get::<ConfMisc>(&esyn)?
            .get();

        let gestures = EsynBuilder::new()
            .set_fn("gestures")
            .flag_res()
            .get::<ConfGestures>(&esyn)?
            .get();

        let layout_double = EsynBuilder::new()
            .set_fn("layout_double")
            .flag_res()
            .get::<ConfLayoutDouble>(&esyn)?
            .get();

        let on_scroll = EsynBuilder::new()
            .set_fn("on_scroll")
            .flag_res()
            .get::<ConfOnScroll>(&esyn)?
            .get();

        let once = EsynBuilder::new()
            .set_fn("once")
            .flag_res()
            .get::<ConfOnce>(&esyn)?
            .get();

        Ok(Self {
            app,
            window,
            canvas,
            page,
            misc,
            once,
            gestures,
            layout_double,
            on_scroll,
        })
    }

    pub fn update(&mut self) -> eyre::Result<()> {
        self.update_file()?;
        // self.update_env()?;
        self.update_cli()?;

        // dbg!(&self);

        Ok(())
    }

    // from config file
    pub fn update_file(&mut self) -> eyre::Result<()> {
        let config = {
            let mut config_path = PathBuf::new();

            // e.g. ~/.config/rmg/config.rs
            let Some(path) = dirs_next::config_dir() else {
                // skip
                // ?create
                return Ok(());
            };

            if path.as_path().is_dir() {
                config_path.push(path.as_path());
                config_path.push("rmg/config.rs");

            // skip
            } else if !config_path.as_path().is_file() {
                return Ok(());
            }

            // tracing::debug!("config_path: {:?}", config_path);

            let mut f = File::open(&config_path)?;
            let mut res = "".to_string();
            f.read_to_string(&mut res)?;

            res
        };

        *self = Self::from_str(&config)?;

        Ok(())
    }

    pub fn update_env(&mut self) {}

    pub fn update_cli(&mut self) -> eyre::Result<()> {
        let mut args = Arguments::from_env();

        // ConfWindow
        if let Some(v) = args.opt_value_from_str::<_, bool>("--window-invert-mouse")? {
            self.window.invert_mouse = v;
        }
        if let Some(v) = args.opt_value_from_str::<_, bool>("--window-borderless")? {
            self.window.borderless = v;
        }

        // ConfCanvas
        if let Some(v) = args.opt_value_from_str::<_, String>("--page-size")? {
            let (w, h) = v.split_once('x').unwrap();
            self.page.size = Size::new(w.parse().unwrap(), h.parse().unwrap());
        }
        if let Some(v) = args.opt_value_from_str::<_, String>("--layout")? {
            self.canvas.layout = {
                match v.to_uppercase().as_str() {
                    "V" | "VERTICAL" | "SCROLL" => Layout::Vertical {
                        align: Align::default(),
                    },
                    "D" | "DOUBLE" => Layout::Double {
                        align: Default::default(),
                        gap: Gap { x: 5.0, y: 10.0 },
                    },
                    "H" | "SINGLE" => Layout::Horizontal { align: Align::Left },
                    "S" | "SINGLE" => Layout::Single {
                        mouse_pos: Vec2::default(),
                        flag_scroll: false,
                        dire: 0.0,
                        cur_zoom: 0,
                        min_zoom: -40,
                        max_zoom: 40,
                    },

                    _ => unimplemented!(),
                }
            };
        }

        // ConfMisc
        if let Some(v) = args.opt_value_from_str::<_, u8>("--padding-filename")? {
            self.misc.padding_filename = v;
        }

        if let Some(v) = args.opt_value_from_str::<_, bool>("--help")? {
            println!("{}", gen_help().as_str());

            exit(0);
        }

        // ConfApp
        self.app.target = args.free_from_str().unwrap();

        Ok(())
    }

    pub fn page_img_resize_algo(&self) -> fir::ResizeAlg {
        self.page.img_resize_algo.into()
    }

    pub fn page_anim_resize_algo(&self) -> fir::ResizeAlg {
        self.page.anim_resize_algo.into()
    }
}

// struct  KeyMap<T = char> {
//     pub up: T,
//     pub down: T,
//     pub left: T,
//     pub right: T,
//     pub exit: T,
//     pub fullscreen: T,
// }

pub fn gen_help() -> String {
    format!(
        r#"
rmg {VERSION}

Tiny And Fast Manga/Image Viewer

USAGE:
    rmg [OPTIONS] [FLAGS] <path>

ARGS:
    <path> A file or directory.

FLAGS:

OPTIONS:
    -h, --help
            Prints help information.
        --config
            Specify the config path.
        --padding-filename
            Padding filename with `0`.

OPTIONS(for Canvas):
        --page-size
            Specify the width and the height of page.
            e.g. `rmg --page-size 900x900`
        --layout
            Specify layout.
            e.g. `rmg --layout double`
"#,
    )
}
