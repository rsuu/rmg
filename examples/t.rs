use dirs_next;

fn main() {
    match dirs_next::config_dir() {
        Some(dir) => {
            println!("{:?}", dir);
        }
        _ => {}
    }
}

use std::{fmt::Display, str::FromStr};

pub trait ValueType<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
}
fn set_value<T: ValueType<String>>(_value: T) {}
