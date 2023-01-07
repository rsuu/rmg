use std::fmt;

impl<AnyType: ?Sized> AutoTrait for AnyType {}

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
