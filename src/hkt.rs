use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}

    impl<T> Sealed for T where T: super::ForLt {}
}

pub trait ForLt: sealed::Sealed {
    type Of<'a>: ?Sized;
}

#[doc(hidden)]
pub trait Hkt<'a> {
    type T: ?Sized;
}

#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct Wrapper<T: ?Sized>(PhantomData<fn(&()) -> &T>);

impl<T: ?Sized + for<'a> Hkt<'a>> Default for Wrapper<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)]
impl<T: ?Sized + for<'a> Hkt<'a>> Wrapper<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[doc(hidden)]
impl<T: ?Sized> ForLt for Wrapper<T> where T: for<'a> Hkt<'a> {
    type Of<'a> = <T as Hkt<'a>>::T;
}
