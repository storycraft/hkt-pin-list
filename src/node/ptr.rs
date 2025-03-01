use core::ptr::NonNull;

use crate::node::{Link, Node};

#[repr(transparent)]
#[derive(Debug)]
pub struct NodePtr<T: ?Sized>(NonNull<Node<T>>);

impl<T: ?Sized> NodePtr<T> {
    pub fn new(node: NonNull<Node<T>>) -> Self {
        Self(node)
    }

    /// # Safety
    /// Pointer must be convertible to a reference
    pub unsafe fn get_extended_ref<'a>(self) -> &'a Node<T> {
        self.0.as_ref()
    }

    /// # Safety
    /// Pointer must be convertible to a reference
    pub unsafe fn as_ref(&self) -> &Node<T> {
        self.0.as_ref()
    }

    /// # Safety
    /// Pointer must be convertible to a reference
    pub unsafe fn link(&self) -> &Link<T> {
        (*self.0.as_ptr()).link.get()
    }

    /// # Safety
    /// Pointer must be convertible to a reference
    pub unsafe fn link_extended<'a>(self) -> &'a Link<T> {
        (*self.0.as_ptr()).link.get()
    }
}

impl<T: ?Sized> Clone for NodePtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for NodePtr<T> {}
