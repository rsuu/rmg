use std::{fs::File, io::Read};

#[derive(Debug)]
struct Config {
    base: Base,
    keymap: Keymap<char>,
}

#[derive(Debug)]
enum ConfigType {
    Base,
    Keymap,
}

#[derive(Debug)]
struct Base {
    size: (usize, usize),
    audio: Option<String>,
    rename: bool,
    rename_pad: usize,
}

#[derive(Debug)]
struct Keymap<_Char> {
    up: _Char,
    down: _Char,
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
            size: (400, 400),
            audio: None,
            rename: true,
            rename_pad: 6,
        }
    }
}

fn main() {
    let content = if let Ok(mut file) = File::open("config.rs") {
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        content
    } else {
        r#"
fn main () {
Base {
size: (900, 900),
audio: "xxx.opus",
rename: false,
rename_pad: 8,
} ;


Keymap { up: 'k', down: 'j' };

}
"#
        .to_string()
    };

    let ast = syn::parse_file(content.as_str()).unwrap();

    //println!("{:#?}", ast);

    // for item in ast.items.iter() {
    // }
    let config = parse_main(ast.items.first().unwrap());
    eprintln!("{:#?}", config);
}

fn parse_main(item: &syn::Item) -> Option<Config> {
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

fn parse_struct(block: &Box<syn::Block>) -> Option<Config> {
    let mut config = Config::default();

    for stmt in block.stmts.iter() {
        match stmt {
            syn::Stmt::Semi(expr, _token) => {
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

fn match_struct_name(expr_struct: &syn::ExprStruct) -> ConfigType {
    let name = expr_struct.path.segments.first().unwrap().ident.to_string();

    match name.as_str() {
        "Base" => ConfigType::Base,
        "Keymap" => ConfigType::Keymap,
        _ => {
            panic!()
        }
    }
}

fn parse_base(expr_struct: &syn::ExprStruct) -> Base {
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

                "audio" => {
                    //eprintln!("{:#?}", _fields);

                    if let syn::Expr::Lit(_expr_lit) = &_fields.expr {
                        if let syn::Lit::Str(_lit_str) = &_expr_lit.lit {
                            base.audio =
                                Some(_lit_str.token().to_string().trim_matches('"').to_string());
                        }
                    }
                }
                "size" => {
                    //eprintln!("{:#?}", _fields);

                    if let syn::Expr::Tuple(tuple_) = &_fields.expr {
                        if let syn::Expr::Lit(_expr) = &tuple_.elems[0] {
                            if let syn::Lit::Int(width) = &_expr.lit {
                                base.size.0 = width.token().to_string().parse::<usize>().unwrap();
                            }
                        }

                        if let syn::Expr::Lit(_expr) = &tuple_.elems[1] {
                            if let syn::Lit::Int(width) = &_expr.lit {
                                base.size.1 = width.token().to_string().parse::<usize>().unwrap();
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

fn parse_keymap(expr_struct: &syn::ExprStruct) -> Keymap<char> {
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
                                .chars().nth(1) // e.g. j
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
                                .chars().nth(1)
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
