pub mod iter;

use core::{
    fmt::Debug,
    pin::{pin, Pin},
    ptr::NonNull,
};

use pin_project_lite::pin_project;

use crate::{hkt::ForLt, node::Next, util::UnsafePinned, Iter};

use super::Node;

pin_project! {
    /// Self managed intrusive linked list
    pub struct LinkedList<Hkt: ForLt> {
        #[pin]
        start: UnsafePinned<Next<Hkt::Of<'static>>>,
    }

    impl<Hkt: ForLt> PinnedDrop for LinkedList<Hkt> {
        fn drop(this: Pin<&mut Self>) {
            // Unlink all entries before dropping list
            this.clear();
        }
    }
}

impl<Hkt: ForLt> LinkedList<Hkt> {
    pub const fn new() -> Self {
        Self {
            start: UnsafePinned::new(Next::new(None)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start.get().get().is_none()
    }

    pub fn push_front(self: Pin<&Self>, node: Pin<&Node<Hkt::Of<'_>>>) {
        node.link(unsafe { &*(&raw const *self.start.get()).cast() });
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

    pub fn iter<R>(&self, f: impl FnOnce(Iter<Hkt::Of<'_>>) -> R) -> R {
        // SAFETY: wrap in closure so nodes cannot drop during iterator is alive
        f(unsafe { Iter::new(self.start.get().get()) })
    }

    pub fn clear(&self) {
        if let Some(start) = self.start.get().take() {
            unsafe { start.link() }.unlink_all();
        }
    }
}

impl<Hkt: ForLt> Default for LinkedList<Hkt> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Hkt: ForLt> Debug for LinkedList<Hkt>
where
    for<'a> Hkt::Of<'a>: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.iter(|iter| f.debug_list().entries(iter).finish())
    }
}

// SAFETY: If List can be moved it is always empty list
unsafe impl<Hkt: ForLt> Send for LinkedList<Hkt> where for<'a> Hkt::Of<'a>: Send {}
