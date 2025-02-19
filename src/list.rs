pub mod iter;

use core::{
    fmt::Debug,
    pin::{pin, Pin},
    ptr::NonNull,
};

use pin_project_lite::pin_project;

use crate::{
    node::{ptr::NodePtr, Next},
    util::UnsafePinned,
    Iter,
};

use super::Node;

pin_project! {
    /// Self managed intrusive linked list
    pub struct LinkedList<T: ?Sized> {
        #[pin]
        start: UnsafePinned<Next<T>>,
    }

    impl<T: ?Sized> PinnedDrop for LinkedList<T> {
        fn drop(this: Pin<&mut Self>) {
            // Unlink all entries before dropping list
            this.clear();
        }
    }
}

impl<T: ?Sized> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            start: UnsafePinned::new(Next::new(None)),
        }
    }

    pub(crate) fn start(&self) -> Option<NodePtr<T>> {
        self.start.get().get()
    }

    pub fn is_empty(&self) -> bool {
        self.start().is_none()
    }

    pub fn push_front(self: Pin<&Self>, node: Pin<&Node<T>>) {
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

    pub fn iter<R>(&self, f: impl FnOnce(Iter<T>) -> R) -> R {
        // SAFETY: wrap in closure so nodes cannot drop during iterator is alive
        f(unsafe { Iter::new(self) })
    }

    pub fn clear(&self) {
        self.iter(|iter| {
            for node in iter {
                node.unlink();
            }
        });
    }
}

impl<T: ?Sized> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized + Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.iter(|iter| f.debug_list().entries(iter).finish())
    }
}
