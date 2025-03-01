#[macro_export]
macro_rules! LinkedList {
    (for<$lt:lifetime> $arg:ty) => {
        $crate::LinkedList<
            $crate::ForLt!(
                for<$lt> $arg
            )
        >
    };

    ($arg:ty) => {
        $crate::LinkedList!(for<'a> $arg)
    };
}

#[macro_export]
macro_rules! ForLt {
    (for<$lt:lifetime> $arg:ty) => {
        $crate::hkt::Wrapper<
            dyn for<$lt> $crate::hkt::Hkt<
                $lt,
                T = $arg
            >
        >
    };

    ($arg:ty) => {
        $crate::ForLt!(for<'a> $arg)
    };
}
