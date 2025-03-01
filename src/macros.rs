#[macro_export]
macro_rules! LinkedList {
    (for<$lt:lifetime> $arg:ty) => {
        $crate::LinkedList<
            $crate::hkt::Wrapper<
                dyn for<$lt> $crate::hkt::Hkt<
                    $lt,
                    T = $arg
                >
            >
        >
    };

    ($arg:ty) => {
        $crate::LinkedList!(for<'a> $arg)
    };
}
