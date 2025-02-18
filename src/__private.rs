pub use core::marker::PhantomData as Pd;

pub use paste::paste;
pub use pin_project_lite::pin_project;

mod inner {
    pub trait Sealed {}
}

pub trait Of<'a>: inner::Sealed {
    type T: ?Sized;
}

impl<'a, T> inner::Sealed for T where T: Of<'a> {}

impl<'a, F, T> Of<'a> for F
where
    T: ?Sized,
    F: FnOnce(&'a ()) -> Pd<T>,
{
    type T = T;
}
