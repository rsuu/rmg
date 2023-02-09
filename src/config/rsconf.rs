use crate::{img::size::Size, render::view::ViewMode, VERSION};
use dirs_next;
use fir;
use lexopt::{self, prelude::*};
use std::{fs::File, io::Read, path::Path, path::PathBuf, process::exit};
use tracing::debug;

#[derive(Debug)]
pub struct Config {
    pub base: Base,
    pub keymap: Keymap<char>,
    pub cli: Cli,
}

#[derive(Debug)]
pub struct Cli {
    pub file_path: Option<String>,
    pub is_debug: bool,
}

#[derive(Debug)]
pub struct Base {
    pub size: Size<u32>,      // window size
    pub font: Option<String>, // font file
    pub rename_pad: u8,       // pad (default: 0)
    pub invert_mouse: bool,   // (default: false)
    pub filter: fir::FilterType,
    pub step: u8,
    pub view_mode: ViewMode,
}

#[derive(Debug)]
pub struct Keymap<Char_> {
    pub up: Char_,    // page up
    pub down: Char_,  // page down
    pub left: Char_,  // move to left
    pub right: Char_, // move to right
    pub exit: Char_,  // exit
    pub fullscreen: Char_,
}

#[derive(Debug)]
pub struct Image {
    pub resize_anim: bool,
    pub resize_bit: bool,
}

#[derive(Debug)]
pub enum ConfigType {
    Base,
    Keymap,
}

impl Config {
    pub fn new() -> Config {
        Config {
            base: Base::default(),
            keymap: Keymap::default(),
            cli: Cli {
                file_path: None,
                is_debug: false,
            },
        }
    }

    fn parse(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let ast = {
            let Ok(mut file) = File::open(path.as_ref()) else { return Err(anyhow::anyhow!("")); };
            let mut code = String::new();
            file.read_to_string(&mut code)?;

            syn::parse_file(code.as_str())?
        };

        if let Some(config) = parse_config(ast.items.first().unwrap()) {
            *self = config;
        } else {
        };

        Ok(())
        // dbg!(ast);
    }

    pub fn try_from_config_file(&mut self) -> anyhow::Result<()> {
        let mut config_path = PathBuf::new();

        // e.g. ~/.config/rmg/config.rs
        let Some(path) = dirs_next::config_dir() else {
            anyhow::bail!("")
        };

        if path.as_path().is_dir() {
            config_path.push(path.as_path());
            config_path.push("rmg/config.rs");
        } else if config_path.as_path().is_file() {
        } else {
            anyhow::bail!("")
        }

        debug!("config_path: {:?}", config_path);

        self.parse(config_path.as_path())
    }

    pub fn try_from_cli(&mut self) -> anyhow::Result<()> {
        let mut parser = lexopt::Parser::from_env();

        while let Some(arg) = parser.next().unwrap() {
            match arg {
                Long("config") | Short('c') => {
                    // parse from file
                    let path = PathBuf::from(parser.value().unwrap().into_string().unwrap());
                    self.parse(path.as_path()).unwrap();
                }

                Long("size") | Short('s') => {
                    let size = parser.value().unwrap().into_string().unwrap();
                    let size = size.as_str().split('x').collect::<Vec<&str>>();

                    let (w, h) = (
                        size[0].parse::<u32>().unwrap_or_default(),
                        size[1].parse::<u32>().unwrap_or_default(),
                    );

                    self.base.size = Size::new(w, h);
                }

                Long("mode") | Short('m') => {
                    let mode = parser.value().unwrap().into_string().unwrap();

                    self.base.view_mode = match mode.as_str() {
                        "s" | "scroll" => ViewMode::Scroll,
                        "o" | "once" => ViewMode::Once,
                        "t" | "turn" => ViewMode::Turn,
                        _ => ViewMode::Scroll,
                    };
                }

                Long("pad") => {
                    let pad = parser.value().unwrap().into_string().unwrap();

                    self.base.rename_pad = pad.parse::<u8>().unwrap();
                }

                Value(v) => {
                    self.cli.file_path = Some(v.into_string().unwrap());
                }

                _ => {
                    print_help();
                }
            }
        }

        Ok(())
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
        Base {
            size: Size::<u32> {
                width: 600,
                height: 600,
            },
            font: None,
            rename_pad: 0,
            invert_mouse: false,
            filter: fir::FilterType::Hamming,
            step: 10,
            view_mode: ViewMode::Scroll,
        }
    }
}

pub fn parse_config(item: &syn::Item) -> Option<Config> {
    // fn main() {
    // ...
    // }

    if let syn::Item::Fn(f) = item {
        if f.sig.ident.to_string().as_str() == "main" {
            parse_struct(&f.block)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn parse_struct(block: &syn::Block) -> Option<Config> {
    let mut config = Config::default();

    for stmt in block.stmts.iter() {
        if let syn::Stmt::Semi(syn::Expr::Struct(expr_struct), _token) = stmt {
            debug!("{:#?}", expr_struct);

            match match_struct_name(expr_struct) {
                ConfigType::Base => {
                    config.base = parse_base(expr_struct);
                }
                ConfigType::Keymap => {
                    config.keymap = parse_keymap(expr_struct);
                }

                _ => {}
            }
        }
    }

    Some(config)
}

// struct NAME {
//   ...
// }
pub fn match_struct_name(expr_struct: &syn::ExprStruct) -> ConfigType {
    let name = expr_struct.path.segments.first().unwrap().ident.to_string();

    match name.as_str() {
        "Base" => ConfigType::Base,
        "Keymap" => ConfigType::Keymap,
        _ => {
            panic!()
        }
    }
}

// BASE {
//   rename_pad,
//   ...
// }
pub fn parse_base(expr_struct: &syn::ExprStruct) -> Base {
    let mut base = Base::default();

    for _fields in expr_struct.fields.iter() {
        let syn::Member::Named(_name) = &_fields.member else {todo!()};

        match (_name.to_string().as_str(), &_fields.expr) {
            ("rename_pad", syn::Expr::Lit(_expr_lit)) => {
                // u8
                // dbg!(_fields);
                let syn::Lit::Int(lit) = &_expr_lit.lit else {panic!()};

                base.rename_pad = lit
                    .token()
                    .to_string()
                    .as_str()
                    .parse::<u8>()
                    .unwrap_or_default(); // default: false
            }

            ("step", syn::Expr::Lit(_expr_lit)) => {
                // u8
                // dbg!(_fields);
                let syn::Lit::Int(lit) = &_expr_lit.lit else {panic!()};

                base.step = lit
                    .token()
                    .to_string()
                    .as_str()
                    .parse::<u8>()
                    .unwrap_or_default(); // default: false
            }

            ("filter", syn::Expr::Lit(_expr_lit)) => {
                // fir::FilterType
                // eprintln!("{:#?}", _fields);

                let syn::Lit::Str(lit) = &_expr_lit.lit else {panic!()};

                let ty = lit.token().to_string().trim_matches('"').to_string();

                debug!("ty: {}", ty);

                base.filter = match ty.as_str() {
                    "Box" => fir::FilterType::Box,
                    "Hamming" => fir::FilterType::Hamming,
                    "CatmullRom" => fir::FilterType::CatmullRom,
                    "Mitchell" => fir::FilterType::Mitchell,
                    "Lanczos3" => fir::FilterType::Lanczos3,
                    _ => fir::FilterType::Hamming,
                };
            }

            ("invert_mouse", syn::Expr::Lit(_expr_lit)) => {
                // bool
                // eprintln!("{:#?}", _fields);

                let syn::Lit::Bool(lit) = &_expr_lit.lit else {panic!()};

                base.invert_mouse = lit
                    .token()
                    .to_string()
                    .as_str()
                    .parse::<bool>()
                    .unwrap_or_default(); // default is false
            }

            ("font", syn::Expr::Lit(_expr_lit)) => {
                //eprintln!("{:#?}", _fields);

                let syn::Lit::Str(lit) = &_expr_lit.lit else {panic!()}
;
                let font_path = lit.token().to_string().trim_matches('"').to_string();

                if std::path::Path::new(font_path.as_str()).is_file() {
                    base.font = Some(font_path);
                } else {
                    base.font = None;
                }
            }

            ("size", syn::Expr::Tuple(tuple_)) => {
                //eprintln!("{:#?}", _fields);

                let syn::Expr::Lit(_lhs) = &tuple_.elems[0] else {panic!()};

                let syn::Expr::Lit(_rhs) = &tuple_.elems[1] else {
                     panic!()
                 };

                if let syn::Lit::Int(width) = &_lhs.lit {
                    base.size.width = width.token().to_string().parse::<u32>().unwrap_or_default();
                }

                if let syn::Lit::Int(height) = &_rhs.lit {
                    base.size.height = height
                        .token()
                        .to_string()
                        .parse::<u32>()
                        .unwrap_or_default();
                }
            }

            _ => {}
        }
    }

    //eprintln!("{:#?}", base);
    base
}

// Keymap {
//   up,
//   ...
// }
pub fn parse_keymap(expr_struct: &syn::ExprStruct) -> Keymap<char> {
    let mut keymap = Keymap::default();

    for _fields in expr_struct.fields.iter() {
        if let syn::Member::Named(_name) = &_fields.member {
            match _name.to_string().as_str() {
                "up" => {
                    // eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Char(_lit_char) = &_expr_lit.lit {
                            keymap.up = _lit_char
                                .token()
                                .to_string()
                                .as_str()
                                .chars()
                                .nth(1) // e.g. j
                                .unwrap();
                        }
                    }
                }

                "down" => {
                    // eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Char(_lit_char) = &_expr_lit.lit {
                            keymap.down = _lit_char
                                .token()
                                .to_string()
                                .as_str()
                                .chars()
                                .nth(1)
                                .unwrap();
                        }
                    }
                }

                "left" => {
                    // eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Char(_lit_char) = &_expr_lit.lit {
                            keymap.left = _lit_char
                                .token()
                                .to_string()
                                .as_str()
                                .chars()
                                .nth(1)
                                .unwrap();
                        }
                    }
                }

                "right" => {
                    // eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Char(_lit_char) = &_expr_lit.lit {
                            keymap.right = _lit_char
                                .token()
                                .to_string()
                                .as_str()
                                .chars()
                                .nth(1)
                                .unwrap();
                        }
                    }
                }

                "exit" => {
                    // eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Char(_lit_char) = &_expr_lit.lit {
                            keymap.exit = _lit_char
                                .token()
                                .to_string()
                                .as_str()
                                .chars()
                                .nth(1)
                                .unwrap();
                        }
                    }
                }

                _ => {}
            }
        }
    }

    //eprintln!("{:#?}", keymap);
    keymap
}

pub fn print_help() -> ! {
    // TODO:
    println!(
        r#"
rmg {VERSION}

Tiny And Fast Manga/Image Viewer

USAGE:
    rmg [OPTIONS] file

OPTIONS:
    -h, --help
            Prints help information
    -V, --version
            Prints version information
    -s, --size
            Reset the width and the height of the buffer
            e.g. rmg --size 900x900
    -c, --config
            Specify the config file path
    -m, --mode
            (TODO)
        --pad
            (TODO)
"#,
    );

    exit(0);
}
