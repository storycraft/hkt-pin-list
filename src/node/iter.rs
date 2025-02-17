use core::marker::PhantomData;

use super::ptr::NodePtr;

#[derive(Debug)]
pub struct Iter<'a> {
    next: Option<NodePtr>,
    _ph: PhantomData<&'a ()>,
}

impl Iterator for Iter<'_> {
    type Item = NodePtr;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.next.take()?;
        self.next = unsafe { v.link().next.get() };
        Some(v)
    }
}

impl Iter<'_> {
    pub(super) unsafe fn new(start: Option<NodePtr>) -> Self {
        Self {
            next: start,
            _ph: PhantomData,
        }
    }
}
