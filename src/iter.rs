use core::marker::PhantomData;

use crate::{node::Node, RawIter};

#[derive(Debug)]
pub struct Iter<'a, T> {
    inner: RawIter,
    _ph: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    #[doc(hidden)]
    pub unsafe fn from(raw: RawIter) -> Self {
        Self {
            inner: raw,
            _ph: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(unsafe { self.inner.next()?.get_extended_ref() })
    }
}
