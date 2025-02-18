pub use pin_project_lite::pin_project;

mod inner {
    pub trait Sealed {}

    pub trait Of<'a> {
        type T;
    }

    impl<'a, F, T> Of<'a> for F
    where
        F: FnOnce(&'a ()) -> T,
    {
        type T = T;
    }

    impl<T> Sealed for T where T: for<'a> Of<'a> {}
}

const _: () = {
    mod inner {}
};

pub trait ForLt: inner::Sealed {
    type Of<'a>;
}

impl<T> ForLt for T
where
    T: for<'a> inner::Of<'a>,
{
    type Of<'a> = <T as inner::Of<'a>>::T;
}

#[macro_export]
#[doc(hidden)]
macro_rules! static_of {
    (for<$($lt:lifetime),*> $ty:ty) => {
        $crate::static_of!(@inner [$($lt),*], $ty)
    };

    (@inner [$lt:lifetime $(,$rest:lifetime)*], $ty:ty) => {
        <for<$lt> fn(&$lt ()) -> $crate::static_of!(@inner [$($rest),*], $ty) as $crate::__private::ForLt>::Of<'static>
    };

    (@inner [], $ty:ty) => {
        <fn(&()) -> $ty as $crate::__private::ForLt>::Of<'static>
    };
}
