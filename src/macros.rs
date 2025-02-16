
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
            pub fn is_empty(self: ::core::pin::Pin<&Self>) -> bool {
                self.project_ref().raw.is_empty()
            }

            /// Link a node to start
            pub fn push_front<$($lt),*>(
                self: ::core::pin::Pin<&Self>,
                entry: &$crate::Entry<$ty>
            ) {
                unsafe {
                    self.project_ref().raw.push_front(entry);
                }
            }

            /// Traverse list from the start until `f` returns true
            pub fn iter<R>(
                self: ::core::pin::Pin<&Self>,
                f: impl for<$($lt),*> ::core::ops::FnOnce(
                    $crate::Iter<'_, $ty>
                ) -> R
            ) -> R
            {
                // SAFETY: hide unbound lifetimes in higher kinded closure
                f(
                    unsafe { $crate::Iter::from(self.project_ref().raw.iter()) }
                )
            }

            pub fn take<R>(
                self: ::core::pin::Pin<&Self>,
                f: impl FnOnce(::core::pin::Pin<&Self>) -> R
            ) -> R {
                // SAFETY: casting transparent struct
                    self.project_ref().raw.take(
                        move |inner| f(
                            unsafe {
                                *(&inner as *const _ as *const ::core::pin::Pin<&Self>)
                            }
                        )
                    )
            }

            /// Clear the list
            pub fn clear(self: ::core::pin::Pin<&Self>) {
                self.project_ref().raw.clear();
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
