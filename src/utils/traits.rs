
use std::fmt;

impl<AnyType: ?Sized> AutoTrait for AnyType {}

use log;
impl<AnyType: ?Sized> AutoLog for AnyType {}

pub trait AutoLog {
    fn _dbg(&self)
    where
        Self: fmt::Display,
    {
        log::debug!("{}", &self);
    }

    fn _info(&self)
    where
        Self: fmt::Display,
    {
        log::info!("{}", &self);
    }

    fn _warn(&self)
    where
        Self: fmt::Display,
    {
        log::warn!("{}", &self);
    }

    fn _err(&self)
    where
        Self: fmt::Display,
    {
        log::error!("{}", &self);
    }

    fn _size() -> usize
    where
        Self: Sized,
    {
        std::mem::size_of::<Self>()
    }
}

pub trait AutoTrait {
    fn k() {}
}

//macro_rules! debug {
//        ($( $args:expr ),*) => {
//
//#[cfg(debug_assertions)]
//{ dbg!( $( $args ),* ); }
//
//#[cfg(not(debug_assertions))]
//{ }
//
//}
//}
