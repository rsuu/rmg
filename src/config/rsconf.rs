use crate::img::size::Size;
use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct Config {
    pub base: Base,
    pub keymap: Keymap<char>,
}

#[derive(Debug)]
pub enum ConfigType {
    Base,
    Keymap,
}

#[derive(Debug)]
pub struct Base {
    pub size: Size<usize>,
    pub font: Option<String>,
    pub rename: bool,
    pub rename_pad: usize,
}

#[derive(Debug)]
pub struct Keymap<_Char> {
    pub up: _Char,
    pub down: _Char,
}

impl Config {
    pub fn parse_from<_Path>(path: &_Path) -> Self
    where
        _Path: AsRef<str> + ?Sized,
    {
        if let Ok(mut file) = File::open(path.as_ref()) {
            let mut content = String::new();

            file.read_to_string(&mut content).unwrap();

            let ast = syn::parse_file(content.as_str()).unwrap();

            //eprintln!("{:#?}", ast);

            // not need now
            // for item in ast.items.iter() {
            // }

            return parse_main(ast.items.first().unwrap()).unwrap();
        } else {
            return Config::default();
        };
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
        Keymap { up: 'k', down: 'j' }
    }
}

impl Default for Base {
    fn default() -> Self {
        Base {
            size: Size::<usize> {
                width: 400,
                height: 400,
            },
            font: None,
            rename: true,
            rename_pad: 6,
        }
    }
}

pub fn parse_main(item: &syn::Item) -> Option<Config> {
    if let syn::Item::Fn(f) = item {
        if f.sig.ident.to_string().as_str() == "main" {
            return parse_struct(&f.block);
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
            syn::Stmt::Semi(expr, token) => {
                match expr {
                    syn::Expr::Struct(expr_struct) => {
                        // TODO
                        //println!("{:#?}", expr_struct);

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
                }
            }
            _ => {}
        }
    }

    Some(config)
}

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

pub fn parse_base(expr_struct: &syn::ExprStruct) -> Base {
    let mut base = Base::default();

    for _fields in expr_struct.fields.iter() {
        if let syn::Member::Named(_name) = &_fields.member {
            match _name.to_string().as_str() {
                "rename" => {
                    //eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Bool(_lit_bool) = &_expr_lit.lit {
                            base.rename = if _lit_bool.token().to_string().as_str() == "true" {
                                true
                            } else if _lit_bool.token().to_string().as_str() == "false" {
                                false
                            } else {
                                panic!("expect bool::[true | false]")
                            };
                        }
                    }
                }
                "rename_pad" => {
                    // eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Int(_lit_int) = &_expr_lit.lit {
                            base.rename_pad = _lit_int
                                .token()
                                .to_string()
                                .as_str()
                                .parse::<usize>()
                                .unwrap_or_default(); // default is 6
                        }
                    }
                }

                "font" => {
                    //eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Str(_lit_str) = &_expr_lit.lit {
                            let font_path =
                                _lit_str.token().to_string().trim_matches('"').to_string();

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
                            if let syn::Lit::Int(width) = &_expr.lit {
                                base.size.width = width
                                    .token()
                                    .to_string()
                                    .parse::<usize>()
                                    .unwrap_or_default();
                            }
                        }

                        if let syn::Expr::Lit(_expr) = &tuple_.elems[1] {
                            if let syn::Lit::Int(width) = &_expr.lit {
                                base.size.height = width
                                    .token()
                                    .to_string()
                                    .parse::<usize>()
                                    .unwrap_or_default();
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
                                .chars() // e.g. 'j'
                                .skip(1) // e.g. j'
                                .next() // e.g. j
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
                                .skip(1)
                                .next()
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

// let a = Some(53);
// let b = Some(4);
// let c = Some(2);
//
// if let Some(d) = (|| {
//     let a = a?;
//     let b = b?;
//     let c = c?;
//     Some(a * b - c)
// })() {
//     println!("{d}");
// }
