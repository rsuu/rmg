fn main() {}

use std::{fmt::Display, str::FromStr};

pub trait ValueType<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
}
fn set_value<T: ValueType<String>>(_value: T) {}
