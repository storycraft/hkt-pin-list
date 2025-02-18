#![no_std]

#[doc(hidden)]
pub mod __private;
mod list;
mod macros;
mod node;
mod util;

pub use list::{iter::Iter, LinkedList};
pub use node::{ptr::NodePtr, Link, Node};

#[cfg(test)]
mod tests {
    use core::pin::pin;

    use super::Node;
    use crate::define_hkt_list;

    #[test]
    fn test() {
        define_hkt_list!(List = &mut i32);

        let mut list = pin!(List::new());
        let list2 = pin!(List::new());
        let list2 = list2.into_ref();

        let mut a = 1234;
        let mut b = 5678;

        let node1 = pin!(Node::new(&mut a));
        let node1 = node1.into_ref();
        let node2 = pin!(Node::new(&mut b));
        let node2 = node2.into_ref();
        list.as_ref().push_front(node2);
        list.as_ref().push_front(node1);

        let list = list.as_mut();

        list2.push_front(node1);
        node1.unlink();
        list.as_ref().push_front(node1);

        list.as_ref().take(|list| {
            list.iter(|mut iter| {
                assert_eq!(
                    iter.next().map(|node| node.get_ref().value()),
                    Some(&&mut 1234)
                );
                let _a = node1;
                let _b = node2;
                assert_eq!(
                    iter.next().map(|node| node.get_ref().value()),
                    Some(&&mut 5678)
                );
                assert_eq!(iter.next().map(|node| node.get_ref().value()), None);
            });
        });
    }
}
