use core::{
    fmt::Debug,
    pin::{pin, Pin},
    ptr::NonNull,
};

use pin_project_lite::pin_project;

use crate::{
    node::{ptr::NodePtr, Next},
    util::UnsafePinned,
    RawIter,
};

use super::Node;

pin_project! {
    /// Raw intrusive hkt linked list
    pub struct RawList {
        #[pin]
        start: UnsafePinned<Next>,
    }

    impl PinnedDrop for RawList {
        fn drop(this: Pin<&mut Self>) {
            // Unlink all entries before dropping list
            this.clear();
        }
    }
}

impl RawList {
    pub fn new() -> Self {
        Self {
            start: UnsafePinned::new(Next::new(None)),
        }
    }

    fn start(&self) -> Option<NodePtr> {
        self.start.get().get()
    }

    pub fn is_empty(&self) -> bool {
        self.start().is_none()
    }

    /// # Safety
    /// Item type must be a same type except lifetimes.
    pub unsafe fn push_front<T>(self: Pin<&Self>, node: Pin<&Node<T>>) {
        let this = self.project_ref();
        let start = this.start.get();
        node.link(start);
    }

    pub fn take<R>(&self, f: impl FnOnce(Pin<&Self>) -> R) -> R {
        let list = pin!(Self::new());
        let list = list.as_ref();

        if let Some(ptr) = self.start.get().take() {
            let new_start = list.project_ref().start;
            let new_start = new_start.get();
            new_start.set(Some(ptr));

            let parent = unsafe { &ptr.link().parent };
            parent.set(Some(NonNull::from(new_start)));
        }

        f(list)
    }

    /// # Safety
    /// Current iterating node must not drop before next iteration
    pub unsafe fn iter(&self) -> RawIter {
        RawIter::new(self.start())
    }

    pub fn clear(&self) {
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
        f.debug_list()
            .entries(unsafe { self.iter() })
            .finish_non_exhaustive()
    }
}
