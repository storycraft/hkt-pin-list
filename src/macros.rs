
#[macro_export]
/// Define a new Safe wrapper around [`raw::RawList`]
macro_rules! define_safe_list {
    ($vis:vis $name:ident = for<$($lt:lifetime),*> $ty:ty) => {
        $crate::__private::pin_project! {
            #[derive(Debug)]
            #[repr(transparent)]    
            $vis struct $name {
                #[pin]
                raw: $crate::RawList,
            }
        }

        #[allow(unused)]
        impl $name {
            /// Create a new List
            pub fn new() -> Self {
                Self {
                    raw: $crate::RawList::new(),
                }
            }

            /// Check if list is empty
            pub fn is_empty(&self) -> bool {
                self.raw.is_empty()
            }

            /// Link a node to start
            pub fn push_front<$($lt),*>(
                self: ::core::pin::Pin<&Self>,
                node: ::core::pin::Pin<&$crate::Node<$ty>>
            ) {
                unsafe {
                    self.project_ref().raw.push_front(node);
                }
            }

            /// Traverse list from the start until `f` returns true
            pub fn iter<R>(
                &self,
                f: impl for<$($lt),*> ::core::ops::FnOnce(
                    $crate::Iter<'_, $ty>
                ) -> R
            ) -> R
            {
                // SAFETY: hide unbound lifetimes in higher kinded closure
                f(
                    unsafe { $crate::Iter::from(self.raw.iter()) }
                )
            }

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
        $crate::define_safe_list!($vis $name = for<> $ty);
    };
}
