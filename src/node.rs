pub mod iter;
pub mod list;
pub mod ptr;

use core::{cell::Cell, pin::Pin, ptr::NonNull};
use pin_project_lite::pin_project;
use pinned_aliasable::Aliasable;
use ptr::EntryPtr;

pub(super) type Next = Cell<Option<EntryPtr>>;
pub(super) type Parent = Cell<Option<NonNull<Next>>>;

pin_project! {
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Node<T> {
        #[pin]
        inner: Aliasable<Entry<T>>,
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Aliasable::new(Entry {
                link: Link {
                    next: Next::new(None),
                    parent: Parent::new(None),
                },
                value,
            }),
        }
    }

    pub fn entry(self: Pin<&Self>) -> &Entry<T> {
        self.project_ref().inner.get()
    }
}

// Node is safe to send if T is Send and not pinned
unsafe impl<T: Send> Send for Node<T> {}

#[repr(C)]
pub struct Entry<T> {
    link: Link,
    value: T,
}

impl<T> Entry<T> {
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_pinned(&self) -> Pin<&T> {
        // SAFETY: Reference to Entry is obtained only from pinned Node
        unsafe { Pin::new_unchecked(&self.value) }
    }

    pub fn linked(&self) -> bool {
        self.link.parent.get().is_some()
    }

    pub fn unlink(&self) {
        self.link.unlink();
    }

    fn link(&self, start: &Next) {
        self.unlink();
        self.link.next.set(start.get());
        self.link.parent.set(Some(NonNull::from(start)));

        if let Some(old) = start.replace(Some(EntryPtr::new(self))) {
            // SAFETY: replace parent of linked start node
            unsafe { old.link() }
                .parent
                .set(Some(NonNull::from(&self.link.next)));
        }
    }
}

impl<T> Drop for Entry<T> {
    fn drop(&mut self) {
        self.unlink();
    }
}

struct Link {
    next: Next,
    parent: Parent,
}

impl Link {
    fn unlink(&self) {
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
