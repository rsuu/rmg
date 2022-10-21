// TODO:
// remove .unwrap()

use fir;
use log;

/* default

fn main() {
    Base {
        size: (900, 900),
        font: None,
        rename_pad: 0,
        invert_mouse: false,
        filter: "Hamming",
        step: 10,
    };

    Keymap {
        up: 'k',
        down: 'j',
        left: 'h',
        right: 'l',
        exit: 'q',
    };
}

*/

use crate::img::size::Size;
use std::{fs::File, io::Read, path::Path};

#[derive(Debug)]
pub struct Config {
    pub base: Base,
    pub keymap: Keymap<char>,
}

#[derive(Debug)]
pub struct Base {
    pub size: Size<usize>,    // window size
    pub font: Option<String>, // font file
    pub rename_pad: u8,       // pad (default: 0)
    pub invert_mouse: bool,   // (default: false)
    pub filter: fir::FilterType,
    pub step: u8,
}

#[derive(Debug)]
pub struct Keymap<Char_> {
    pub up: Char_,    // page up
    pub down: Char_,  // page down
    pub left: Char_,  // move to left
    pub right: Char_, // move to right
    pub exit: Char_,  // exit
                      //pub fullscreen: Char_,
}

#[derive(Debug)]
pub enum ConfigType {
    Base,
    Keymap,
}

impl Config {
    pub fn parse_from<_Path>(path: &_Path) -> Self
    where
        _Path: AsRef<Path> + ?Sized,
    {
        if let Ok(mut file) = File::open(path.as_ref()) {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let ast = syn::parse_file(content.as_str()).unwrap();

            //eprintln!("{:#?}", ast);

            // not need now
            // for item in ast.items.iter() {}

            return parse_main(ast.items.first().unwrap()).unwrap();
        } else {
            Config::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base: Base::default(),
            keymap: Keymap::default(),
        }
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
        }
    }
}

impl Default for Base {
    fn default() -> Self {
        Base {
            size: Size::<usize> {
                width: 900,
                height: 900,
            },
            font: None,
            rename_pad: 0,
            invert_mouse: false,
            filter: fir::FilterType::Hamming,
            step: 10,
        }
    }
}

pub fn parse_main(item: &syn::Item) -> Option<Config> {
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

pub fn parse_struct(block: &Box<syn::Block>) -> Option<Config> {
    let mut config = Config::default();

    for stmt in block.stmts.iter() {
        match stmt {
            syn::Stmt::Semi(expr, _token) => match expr {
                syn::Expr::Struct(expr_struct) => {
                    log::debug!("{:#?}", expr_struct);

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
                _ => {}
            },
            _ => {}
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
        if let syn::Member::Named(_name) = &_fields.member {
            match _name.to_string().as_str() {
                // u8
                "rename_pad" => {
                    // eprintln!("{:#?}", _fields);
                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Int(lit) = &_expr_lit.lit {
                            base.rename_pad = lit
                                .token()
                                .to_string()
                                .as_str()
                                .parse::<u8>()
                                .unwrap_or_default(); // default: false
                        }
                    }
                }

                // u8
                "step" => {
                    // eprintln!("{:#?}", _fields);
                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Int(lit) = &_expr_lit.lit {
                            base.step = lit
                                .token()
                                .to_string()
                                .as_str()
                                .parse::<u8>()
                                .unwrap_or_default(); // default: false
                        }
                    }
                }

                // fir::FilterType
                "filter" => {
                    // eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Str(lit) = &_expr_lit.lit {
                            let ty = lit.token().to_string().trim_matches('"').to_string();

                            log::debug!("ty: {}", ty);

                            base.filter = match ty.as_str() {
                                "Box" => fir::FilterType::Box,
                                "Hamming" => fir::FilterType::Hamming,
                                "CatmullRom" => fir::FilterType::CatmullRom,
                                "Mitchell" => fir::FilterType::Mitchell,
                                "Lanczos3" => fir::FilterType::Lanczos3,
                                _ => fir::FilterType::Hamming,
                            };
                        }
                    }
                }

                // bool
                "invert_mouse" => {
                    // eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Bool(lit) = &_expr_lit.lit {
                            base.invert_mouse = lit
                                .token()
                                .to_string()
                                .as_str()
                                .parse::<bool>()
                                .unwrap_or_default(); // default is false
                        }
                    }
                }

                "font" => {
                    //eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Str(lit) = &_expr_lit.lit {
                            let font_path = lit.token().to_string().trim_matches('"').to_string();

                            if std::path::Path::new(font_path.as_str()).is_file() {
                                base.font = Some(font_path);
                            } else {
                                base.font = None;
                            }
                        }
                    }
                }

                "size" => {
                    //eprintln!("{:#?}", _fields);

                    if let syn::Expr::Tuple(tuple_) = &_fields.expr {
                        if let syn::Expr::Lit(_expr) = &tuple_.elems[0] {
                            if let syn::Lit::Int(lit) = &_expr.lit {
                                base.size.width =
                                    lit.token().to_string().parse::<usize>().unwrap_or_default();
                            }
                        }

                        if let syn::Expr::Lit(_expr) = &tuple_.elems[1] {
                            if let syn::Lit::Int(lit) = &_expr.lit {
                                base.size.height =
                                    lit.token().to_string().parse::<usize>().unwrap_or_default();
                            }
                        }
                    }
                }

                _ => {}
            }
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
