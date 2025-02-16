use core::ptr::NonNull;

use crate::node::{Link, Entry};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct EntryPtr(NonNull<Entry<()>>);

impl EntryPtr {
    pub fn new<T>(node: &Entry<T>) -> Self {
        Self(NonNull::from(node).cast())
    }

    /// # Safety
    /// Pointer must be convertible to a reference 
    pub unsafe fn get_extended_ref<'a, T>(self) -> &'a Entry<T> {
        self.0.cast().as_ref()
    }

    /// # Safety
    /// Pointer must be convertible to a reference
    pub unsafe fn as_ref<T>(&self) -> &Entry<T> {
        self.0.cast().as_ref()
    }

    pub(super) unsafe fn link(&self) -> &Link {
        &(*self.0.as_ptr()).link
    }
}
