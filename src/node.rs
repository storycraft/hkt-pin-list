pub mod iter;
pub mod list;
pub mod ptr;

use core::{cell::Cell, pin::Pin, ptr::NonNull};
use pin_project_lite::pin_project;
use ptr::NodePtr;

use crate::util::UnsafePinned;

pub(super) type Next = Cell<Option<NodePtr>>;
pub(super) type Parent = Cell<Option<NonNull<Next>>>;

pin_project! {
    #[derive(Debug)]
    // Use repr(C) to ensure `link` is accessible from generic erased pointer
    #[repr(C)]
    pub struct Node<T: ?Sized> {
        #[pin]
        link: UnsafePinned<Link>,
        #[pin]
        value: UnsafePinned<T>,
    }

    impl<T: ?Sized> PinnedDrop for Node<T> {
        fn drop(this: Pin<&mut Self>) {
            this.link.get().unlink();
        }
    }
}

impl<T: ?Sized> Node<T> {
    pub fn new(value: T) -> Self where T: Sized {
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

    fn link(self: Pin<&Self>, start: &Next) {
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

// Node is safe to send if T is Send and not pinned
unsafe impl<T: ?Sized + Send> Send for Node<T> {}

#[derive(Debug)]
pub struct Link {
    next: Next,
    parent: Parent,
}

impl Link {
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
}
