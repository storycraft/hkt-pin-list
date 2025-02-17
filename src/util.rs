use core::{fmt::Debug, marker::PhantomPinned, pin::Pin};

use pin_project_lite::pin_project;

pin_project! {
    /// UnsafePinned at home
    ///
    /// See: <https://github.com/rust-lang/rust/pull/82834>
    #[repr(transparent)]
    pub struct UnsafePinned<T> {
        #[pin]
        inner: T,
        _pinned: PhantomPinned,
    }
}

impl<T> UnsafePinned<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            _pinned: PhantomPinned,
        }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    pub fn get_pinned(self: Pin<&Self>) -> Pin<&T> {
        self.project_ref().inner
    }
}

impl<T: Debug> Debug for UnsafePinned<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UnsafePinned").field(&self.inner).finish()
    }
}
