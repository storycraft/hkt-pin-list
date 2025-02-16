use core::marker::PhantomData;

use super::ptr::EntryPtr;

#[derive(Debug)]
pub struct Iter<'a> {
    next: Option<EntryPtr>,
    _ph: PhantomData<&'a ()>,
}

impl Iterator for Iter<'_> {
    type Item = EntryPtr;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.next.take()?;
        self.next = unsafe { v.link().next.get() };
        Some(v)
    }
}

impl Iter<'_> {
    pub(super) unsafe fn new(start: Option<EntryPtr>) -> Self {
        Self {
            next: start,
            _ph: PhantomData,
        }
    }
}
