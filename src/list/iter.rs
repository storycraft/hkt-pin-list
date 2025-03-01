use super::Node;
use crate::NodePtr;
use core::{marker::PhantomData, pin::Pin};

#[derive(Debug)]
pub struct Iter<'a, T: ?Sized + 'a> {
    next: Option<NodePtr<T>>,
    _ph: PhantomData<&'a ()>,
}

impl<'a, T: ?Sized + 'a> Iterator for Iter<'a, T> {
    type Item = Pin<&'a Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.next.take()?;
        self.next = unsafe { v.link().next.get() };
        Some(unsafe { Pin::new_unchecked(v.get_extended_ref()) })
    }
}

impl<'a, T: ?Sized + 'a> Iter<'a, T> {
    pub(super) unsafe fn new(start: Option<NodePtr<T>>) -> Self {
        Self {
            next: start,
            _ph: PhantomData,
        }
    }
}
