#![no_std]

#[doc(hidden)]
pub mod __private;
mod iter;
mod macros;
mod node;

pub use iter::Iter;
pub use node::{iter::Iter as RawIter, list::RawList, ptr::EntryPtr, Entry, Node};

#[cfg(test)]
mod tests {
    use core::pin::pin;

    use super::Node;
    use crate::{define_safe_list, node::Entry};

    #[test]
    fn test() {
        define_safe_list!(List = &mut i32);

        let mut list = pin!(List::new());
        let list2 = pin!(List::new());
        let list2 = list2.into_ref();

        let mut a = 1234;
        let mut b = 5678;

        let node1 = pin!(Node::new(&mut a));
        let node1 = node1.into_ref();
        let node2 = pin!(Node::new(&mut b));
        let node2 = node2.into_ref();
        list.as_ref().push_front(node2.entry());
        list.as_ref().push_front(node1.entry());

        let list = list.as_mut();

        list2.push_front(node1.entry());
        node1.entry().unlink();
        list.as_ref().push_front(node1.entry());

        list.as_ref().take(|list| {
            list.iter(|mut iter| {
                assert_eq!(iter.next().map(Entry::value), Some(&&mut 1234));
                let _a = node1;
                let _b = node2;
                assert_eq!(iter.next().map(Entry::value), Some(&&mut 5678));
                assert_eq!(iter.next().map(Entry::value), None);
            });
        });
    }
}
