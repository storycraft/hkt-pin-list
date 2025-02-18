use core::{marker::PhantomData, pin::Pin};

use crate::{node::Node, RawIter};

#[derive(Debug)]
pub struct Iter<'a, T: ?Sized> {
    inner: RawIter,
    _ph: PhantomData<&'a T>,
}

impl<T: ?Sized> Iter<'_, T> {
    #[doc(hidden)]
    pub unsafe fn from(raw: RawIter) -> Self {
        Self {
            inner: raw,
            _ph: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Pin<&'a Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(unsafe { Pin::new_unchecked(self.inner.next()?.get_extended_ref()) })
    }
}
