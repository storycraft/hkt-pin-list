#[macro_export]
/// Define a new hkt wrapper around `List`
macro_rules! define_hkt_list {
    ($vis:vis $name:ident = for<$($lt:lifetime),*> $ty:ty) => {
        #[derive(Debug)]
        #[repr(transparent)]
        $vis struct $name {
            raw: $crate::LinkedList<$crate::static_of!(for<$($lt),*> $ty)>,
        }

        #[allow(unused)]
        impl $name {
            /// Create a new List
            #[inline(always)]
            pub const fn new() -> Self {
                Self {
                    raw: $crate::LinkedList::new(),
                }
            }

            /// Check if list is empty
            #[inline(always)]
            pub fn is_empty(&self) -> bool {
                self.raw.is_empty()
            }

            /// Link a node to start
            #[inline(always)]
            pub fn push_front<$($lt),*>(
                self: ::core::pin::Pin<&Self>,
                node: ::core::pin::Pin<&$crate::Node<$ty>>
            ) {
                // SAFETY: projection and extend lifetime
                unsafe {
                    ::core::pin::Pin::new_unchecked(
                        &self.raw
                    ).push_front(::core::mem::transmute(node));
                }
            }

            /// Create iterator
            #[inline(always)]
            pub fn iter<R>(
                &self,
                f: impl for<$($lt),*> ::core::ops::FnOnce(
                    $crate::Iter<'_, $ty>
                ) -> R
            ) -> R
            {
                self.raw.iter(f)
            }

            #[inline(always)]
            pub fn take<R>(
                &self,
                f: impl FnOnce(::core::pin::Pin<&Self>) -> R
            ) -> R {
                // SAFETY: casting transparent struct
                    self.raw.take(
                        move |inner| f(
                            unsafe {
                                *(&inner as *const _ as *const ::core::pin::Pin<&Self>)
                            }
                        )
                    )
            }

            /// Clear the list
            #[inline(always)]
            pub fn clear(&self) {
                self.raw.clear();
            }
        }

        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };

    ($vis:vis $name:ident = $ty:ty) => {
        $crate::define_hkt_list!($vis $name = for<> $ty);
    };
}
