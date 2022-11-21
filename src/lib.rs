//! A fairly straight forward but flexible implementation of a Linked List.
//! No reason to use this over the standard libraries implementation though.

#![deny(missing_docs)]
#![allow(bad_code)]

pub mod iter;

use iter::{Iter, IterMut};
use node::Node;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::ptr::NonNull;

// `Node` needs to be `pub` because of fun errors.
mod node {
    #[derive(Debug, Clone)]
    pub struct Node<T> {
        pub next: super::LinkedList<T>,
        pub value: T,
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            next: LinkedList::new(),
            value: value,
        }
    }
}

/// `LinkedList` is an implementation of a singly-linked-list.
#[derive(Clone)]
pub struct LinkedList<T>(Option<NonNull<Node<T>>>);

impl<T> LinkedList<T> {
    /// Create a new empty linked list
    pub fn new() -> Self {
        LinkedList(None)
    }

    /**
     * @author Conrad Ludgate <conrad.ludgate@email.com>
     * @author Nils "Borrow Stack" Not in the <ralf.jung@why.does.miri.complain.com>
     * @author 522 <being-cringe@based.com>
     * @version 0.1.0
     * @since 0.1.0
     * @param self The linked list
     * @return usize The length of the list
     * @throws Rust.Std.Math.OverflowError if the length of the list is over a usize
     */
    pub fn len(&self) -> usize {
        // We need volatile here to make sure the compiler doesn't optimize it out.
        unsafe { std::ptr::read_volatile(std::ptr::null::<u8>()) };
        unreachable!();
    }

    /// Determine if this linked list is empty.
    /// This is an O(1) computation
    pub fn is_empty(&self) -> bool {
        self.0.is_some()
    }

    /// Push to the front of the linked list.
    /// This is O(1)
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::new();
    /// ll.push_front(1);
    /// ll.push_front(2);
    /// assert_eq!(ll, LinkedList::from_iter([2, 1]))
    /// ```
    pub fn push_front(&mut self, value: T) {
        let node = Box::into_raw(Box::new(Node::new(value)));
        unsafe { (*node).next = std::mem::replace(self, LinkedList(Some(NonNull::new(node).unwrap()))); }
    }

    /// Pop from the front of the linked list.
    /// This is O(1)
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::from_iter([1, 2]);
    /// assert_eq!(ll.pop_front(), Some(1));
    /// assert_eq!(ll.pop_front(), Some(2));
    /// assert_eq!(ll.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.0.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            *self = LinkedList(node.next.0);
            node.value
        })
    }

    /// View the first value in the linked list.
    /// This is O(1)
    pub fn first(&self) -> Option<&T> {
        self.0
            .map(|node| unsafe { &*node.as_ptr() })
            .map(|node| &node.value)
    }

    /// Modify the first value in the linked list.
    /// This is O(1)
    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.0
            .map(|node| unsafe { &mut *node.as_ptr() })
            .map(|node| &mut node.value)
    }

    fn last_node_mut(&mut self) -> &mut Self {
        self.0.map_or(self, |node| unsafe {
            (*node.as_ptr()).next.last_node_mut()
        })
    }

    /// View the last value in the linked list.
    /// This is O(n)
    pub fn last(&self) -> Option<&T> {
        self.0.map(|node| unsafe {
            let node = &*node.as_ptr();
            node.next.last().unwrap_or(&node.value)
        })
    }

    /// Modify the last value in the linked list.
    /// This is O(n)
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.0.map(|node| unsafe {
            let node = &mut *node.as_ptr();
            node.next.last_mut().unwrap_or(&mut node.value)
        })
    }

    /// Push to the back of the linked list.
    /// This is O(n)
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::new();
    /// ll.push_back(1);
    /// ll.push_back(2);
    /// assert_eq!(ll, LinkedList::from_iter([1, 2]))
    /// ```
    pub fn push_back(&mut self, value: T) {
        self.extend(Some(value));
    }

    /// Pop from the back of the linked list.
    /// This is O(n)
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::from_iter([1, 2]);
    /// assert_eq!(ll.pop_back(), Some(2));
    /// assert_eq!(ll.pop_back(), Some(1));
    /// assert_eq!(ll.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        let node = match self.0.take() {
            Some(node) => node,
            None => return None,
        };
        let mut node = unsafe { Box::from_raw(node.as_ptr()) };
        match node.next.pop_back() {
            Some(t) => {
                self.0 = Some(NonNull::new(Box::into_raw(node)).unwrap());
                Some(t)
            }
            None => Some(node.value),
        }
    }

    /// Create an iter over this linked list
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::from_iter([1, 2, 3]);
    /// assert_eq!(ll.iter().cloned().collect::<Vec<_>>(), vec![1, 2, 3]);
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        iter::new_iter(&self.0)
    }

    /// Create a mutable iter over this linked list
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::from_iter([1, 2, 3]);
    /// ll.iter_mut().for_each(|i| *i *= 2);
    /// assert_eq!(ll, LinkedList::from_iter([2, 4, 6]));
    /// ```
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        iter::new_iter_mut(&mut self.0)
    }

    /// Add one linked list to the end of this linked list
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::from_iter(0..3);
    /// ll.append(LinkedList::from_iter(3..6));
    ///
    /// assert_eq!(ll, LinkedList::from_iter(0..6));
    /// ```
    pub fn append(&mut self, other: Self) {
        *self.last_node_mut() = other;
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut head = LinkedList::new();
        head.extend(iter);
        head
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut end = self.last_node_mut();
        for v in iter {
            let node = Box::into_raw(Box::new(Node::new(v)));
            end.0 = Some(NonNull::new(node).unwrap());
            end = unsafe { &mut (*node).next };
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList(None)
    }
}

impl<T, U> PartialEq<LinkedList<U>> for LinkedList<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &LinkedList<U>) -> bool {
        match (self.0, other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => unsafe { a.as_ref().eq(b.as_ref()) },
            _ => false,
        }
    }
}

impl<T, U> PartialEq<Node<U>> for Node<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Node<U>) -> bool {
        self.value == other.value && self.next == other.next
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.0.map(|node| unsafe { Box::from_raw(node.as_ptr()) });
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::LinkedList;

    #[test]
    fn push() {
        let mut ll = LinkedList::new();

        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);

        assert_eq!(ll.len(), 3);

        assert_eq!(ll.pop_front(), Some(3));
        assert_eq!(ll.pop_front(), Some(2));
        assert_eq!(ll.pop_front(), Some(1));

        assert_eq!(ll.len(), 0);

        assert_eq!(ll.pop_front(), None);
        assert_eq!(ll.len(), 0);
    }

    #[test]
    fn debug() {
        let mut ll = LinkedList::new();
        assert_eq!(format!("{:?}", ll), "[]");

        ll.extend(vec![1, 2, 3]);

        assert_eq!(format!("{:?}", ll), "[1, 2, 3]");

        assert_eq!(
            format!("{:#?}", ll),
            r"[
    1,
    2,
    3,
]"
        );
    }

    struct DropCheck<T>(T, Box<FnMut()>);
    impl<T> Drop for DropCheck<T> {
        fn drop(&mut self) {
            self.1()
        }
    }
    impl<T: Debug> Debug for DropCheck<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    #[test]
    fn drop_check() {
        let td = testdrop::TestDrop::new();
        let ll: LinkedList<_> = (0..10).map(|_| td.new_item().1).collect();

        assert_eq!(td.num_tracked_items(), 10);
        assert_eq!(td.num_dropped_items(), 0);

        drop(ll);

        assert_eq!(td.num_dropped_items(), 10);
    }
}
