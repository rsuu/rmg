use crate::utils::types::{SelfResult};
use std::io::{self, Read};

fn from_pipe() -> SelfResult<String> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    Ok(buffer)
}
