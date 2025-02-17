use core::ptr::NonNull;

use crate::node::{Link, Node};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct NodePtr(NonNull<Node<()>>);

impl NodePtr {
    pub fn new<T>(node: &Node<T>) -> Self {
        Self(NonNull::from(node).cast())
    }

    /// # Safety
    /// Pointer must be convertible to a reference
    pub unsafe fn get_extended_ref<'a, T>(self) -> &'a Node<T> {
        self.0.cast().as_ref()
    }

    /// # Safety
    /// Pointer must be convertible to a reference
    pub unsafe fn as_ref<T>(&self) -> &Node<T> {
        self.0.cast().as_ref()
    }

    /// # Safety
    /// Pointer must be convertible to a reference
    pub unsafe fn link(&self) -> &Link {
        (*self.0.as_ptr()).link.get()
    }
}
