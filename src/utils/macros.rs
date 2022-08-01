// block | expr | ident | item | lifetime | literal
// meta | pat | pat_param | path | stmt | tt | ty | vis

// not need now
#[macro_export]
macro_rules! unwrap_or_return {
    ( $e:expr , $err:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Err($err(e)),
        }
    };
}

#[macro_export]
macro_rules! impl_from {
    ( $($l:path, $r:path;)* ) => {
        $(
            impl From<$l> for MyError {
                fn from(e: $l) -> Self {
                    $r(e)
                }
            }
            )*
    }
}

#[macro_export]
macro_rules! check {
   ( $( $args:expr ),* ) => {
       #[cfg(debug_assertions)]
       {
           dbg!( $( $args ),* );
       }

       #[cfg(not(debug_assertions))]
       { }
   }
}
