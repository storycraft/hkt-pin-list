use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}

    impl<T> Sealed for T where T: super::ForLt {}
}

pub trait ForLt: sealed::Sealed {
    type Of<'a>: ?Sized;
}

pub trait Hkt<'a> {
    type T: ?Sized;
}

#[derive(Debug, Clone, Copy)]
pub struct Wrapper<T: ?Sized>(PhantomData<fn(&()) -> &T>);

impl<T: ?Sized + for<'a> Hkt<'a>> Default for Wrapper<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized + for<'a> Hkt<'a>> Wrapper<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: ?Sized> ForLt for Wrapper<T> where T: for<'a> Hkt<'a> {
    type Of<'a> = <T as Hkt<'a>>::T;
}
