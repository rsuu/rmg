use log;
use std::fmt;

impl<AnyType: ?Sized> AutoTrait for AnyType {}

impl<AnyType: ?Sized + std::fmt::Display> AutoLog for AnyType {}

pub trait AutoLog
where
    Self: fmt::Display,
{
    fn _dbg(&self) {
        log::debug!("{}", &self);
    }

    fn _info(&self) {
        log::info!("{}", &self);
    }

    fn _warn(&self) {
        log::warn!("{}", &self);
    }

    fn _err(&self) {
        log::error!("{}", &self);
    }
}

pub trait AutoTrait {
    // usage:
    //     <u8>::_size()
    //     <Option<u32>>::_size()
    fn _size() -> usize
    where
        Self: Sized,
    {
        std::mem::size_of::<Self>()
    }
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
