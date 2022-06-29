// not need now
macro_rules! unwrap_or_return {
    ( $e:expr , $err:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Err($err(e)),
        }
    };
}
