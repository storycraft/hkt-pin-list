#![no_std]

mod list;
mod macros;
mod node;
mod util;
pub mod hkt;

pub use list::{iter::Iter, LinkedList};
pub use node::{ptr::NodePtr, Link, Node};

#[cfg(test)]
mod tests {
    use core::pin::pin;

    use crate::LinkedList;

    use super::Node;

    extern crate alloc;

    #[test]
    fn test() {
        let mut list = pin!(<LinkedList!(i32)>::new());
        let list2 = pin!(<LinkedList!(i32)>::new());
        let list2 = list2.into_ref();

        let node1 = pin!(Node::new(1234));
        let node1 = node1.into_ref();
        let node2 = pin!(Node::new(5678));
        let node2 = node2.into_ref();
        list.as_ref().push_front(node2);
        list.as_ref().push_front(node1);

        let list = list.as_mut();

        list2.push_front(node1);
        node1.unlink();
        list.as_ref().push_front(node1);

        list.as_ref().take(|list| {
            list.iter(|mut iter| {
                assert_eq!(iter.next().map(|node| node.get_ref().value()), Some(&1234));
                let _a = node1;
                let _b = node2;
                assert_eq!(iter.next().map(|node| node.get_ref().value()), Some(&5678));
                assert_eq!(iter.next().map(|node| node.get_ref().value()), None);
            });
        });
    }
}
