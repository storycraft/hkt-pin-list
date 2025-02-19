pub mod ptr;

use core::{cell::Cell, fmt::Debug, pin::Pin, ptr::NonNull};
use pin_project_lite::pin_project;
use ptr::NodePtr;

use crate::util::UnsafePinned;

pub(super) type Next<T> = Cell<Option<NodePtr<T>>>;
pub(super) type Parent<T> = Cell<Option<NonNull<Next<T>>>>;

pin_project! {
    pub struct Node<T: ?Sized, Dyn: ?Sized = T> {
        #[pin]
        link: UnsafePinned<Link<Dyn>>,
        #[pin]
        value: UnsafePinned<T>,
    }

    impl<T: ?Sized, Dyn: ?Sized> PinnedDrop for Node<T, Dyn> {
        fn drop(this: Pin<&mut Self>) {
            this.link.get().unlink();
        }
    }
}

impl<T: ?Sized, Dyn: ?Sized> Node<T, Dyn> {
    pub fn new(value: T) -> Self
    where
        T: Sized + 'static,
    {
        unsafe { Self::new_unchecked(value) }
    }

    /// # Safety
    /// You must ensure the destructor has run
    pub unsafe fn new_unchecked(value: T) -> Self
    where
        T: Sized,
    {
        Self {
            link: UnsafePinned::new(Link {
                next: Next::new(None),
                parent: Parent::new(None),
            }),
            value: UnsafePinned::new(value),
        }
    }

    pub fn value(&self) -> &T {
        self.value.get()
    }

    pub fn value_pinned(self: Pin<&Self>) -> Pin<&T> {
        self.project_ref().value.get_pinned()
    }

    pub fn linked(&self) -> bool {
        self.link.get().linked()
    }

    pub fn unlink(&self) {
        self.link.get().unlink();
    }
}

impl<Dyn: ?Sized> Node<Dyn> {
    pub(super) fn link(self: Pin<&Self>, start: &Next<Dyn>) {
        self.unlink();
        let link = self.link.get();
        link.next.set(start.get());
        link.parent.set(Some(NonNull::from(start)));

        if let Some(old) = start.replace(Some(NodePtr::new(&self))) {
            // SAFETY: replace parent of linked start node
            unsafe { old.link() }
                .parent
                .set(Some(NonNull::from(&link.next)));
        }
    }
}

impl<T: ?Sized + Debug, Dyn: ?Sized> Debug for Node<T, Dyn> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Node")
            .field("linked", &self.linked())
            .field("value", &self.value.get())
            .finish()
    }
}

// Node is safe to send if T is Send and not pinned
unsafe impl<T: ?Sized + Send, Dyn: ?Sized + Send> Send for Node<T, Dyn> {}

#[derive(Debug)]
pub struct Link<T: ?Sized> {
    pub(super) next: Next<T>,
    pub(super) parent: Parent<T>,
}

impl<T: ?Sized> Link<T> {
    pub fn linked(&self) -> bool {
        self.parent.get().is_some()
    }

    pub fn unlink(&self) {
        if let Some(parent) = self.parent.take() {
            let next = self.next.take();

            if let Some(ref next) = next {
                // SAFETY: pointer is valid as long as linked
                unsafe {
                    next.link().parent.set(Some(parent));
                }
            }

            // SAFETY: pointer is valid as long as linked
            unsafe { parent.as_ref() }.set(next);
        }
    }

    pub(super) fn unlink_all(&self) {
        let mut link = self;
        while let Some(_) = link.parent.take() {
            let Some(ptr) = link.next.take() else {
                break;
            };

            // SAFETY: If linked, next is valid pointer
            link = unsafe { ptr.link_extended() };
        }
    }
}
