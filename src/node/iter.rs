use super::ptr::NodePtr;

#[derive(Debug)]
pub struct RawIter {
    next: Option<NodePtr>,
}

impl Iterator for RawIter {
    type Item = NodePtr;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.next.take()?;
        self.next = unsafe { v.link().next.get() };
        Some(v)
    }
}

impl RawIter {
    pub(super) unsafe fn new(start: Option<NodePtr>) -> Self {
        Self {
            next: start,
        }
    }
}
