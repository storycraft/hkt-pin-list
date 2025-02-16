use core::{
    fmt::Debug,
    pin::{pin, Pin},
    ptr::NonNull,
};

use pin_project_lite::pin_project;
use pinned_aliasable::Aliasable;

use crate::{
    node::{ptr::EntryPtr, Next},
    RawIter,
};

use super::Entry;

pin_project! {
    /// Raw intrusive hkt linked list
    pub struct RawList {
        #[pin]
        start: Aliasable<Next>,
    }

    impl PinnedDrop for RawList {
        fn drop(this: Pin<&mut Self>) {
            // Unlink all entries before dropping list
            this.into_ref().clear();
        }
    }
}

impl RawList {
    pub fn new() -> Self {
        Self {
            start: Aliasable::new(Next::new(None)),
        }
    }

    fn start(self: Pin<&Self>) -> Option<EntryPtr> {
        self.project_ref().start.get().get()
    }

    pub fn is_empty(self: Pin<&Self>) -> bool {
        self.start().is_none()
    }

    /// # Safety
    /// Item type must be a same type except lifetimes.
    pub unsafe fn push_front<T>(self: Pin<&Self>, entry: &Entry<T>) {
        let start = self.project_ref().start.get();
        entry.link(start);
    }

    pub fn take<R>(self: Pin<&Self>, f: impl FnOnce(Pin<&Self>) -> R) -> R {
        let list = pin!(Self::new());
        let list = list.as_ref();

        if let Some(ptr) = self.project_ref().start.get().take() {
            let new_start = list.project_ref().start.get();
            new_start.set(Some(ptr));

            let parent = unsafe { &ptr.link().parent };
            parent.set(Some(NonNull::from(new_start)));
        }

        f(list)
    }

    /// # Safety
    /// Items must not drop during iteration
    pub unsafe fn iter(self: Pin<&Self>) -> RawIter {
        RawIter::new(self.start())
    }

    pub fn clear(self: Pin<&Self>) {
        for ptr in unsafe { self.iter() } {
            unsafe { ptr.link() }.unlink();
        }
    }
}

impl Default for RawList {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for RawList {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().finish_non_exhaustive()
    }
}
