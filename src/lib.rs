pub mod iter;

use std::{fmt::Debug, ptr::NonNull};

use crate::iter::{Iter, IterMut};

#[derive(Debug, Clone)]
pub struct Node<T> {
    next: LinkedList<T>,
    value: T,
}

impl<T> Node<T> {
    const fn new(value: T) -> Self {
        Self {
            next: LinkedList::new(),
            value,
        }
    }
}

/// LinkedList is an implementation of a singly-linked-list.
#[derive(Clone)]
pub struct LinkedList<T>(pub(crate) Option<NonNull<Node<T>>>);

impl<T> LinkedList<T> {
    /// Create a new empty linked list
    pub const fn new() -> Self {
        Self(None)
    }

    /// Get the length of the linked list.
    /// This is an O(n) computation
    pub fn len(&self) -> usize {
        self.0
            .as_ref()
            .map_or(0, |node| unsafe { node.as_ref().next.len() + 1 })
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
        let node = Box::leak(Box::new(Node::new(value)));
        node.next = std::mem::replace(self, Self(Some(node.into())));
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
            *self = node.next;
            node.value
        })
    }

    /// View the first value in the linked list.
    /// This is O(1)
    pub fn first(&self) -> Option<&T> {
        self.0
            .map(|node| unsafe { node.as_ref() })
            .map(|node| &node.value)
    }

    /// Modify the first value in the linked list.
    /// This is O(1)
    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.0
            .map(|mut node| unsafe { node.as_mut() })
            .map(|node| &mut node.value)
    }

    fn last_node_mut(&mut self) -> &mut Self {
        self.0.map_or(self, |mut node| unsafe {
            node.as_mut().next.last_node_mut()
        })
    }

    /// View the last value in the linked list.
    /// This is O(n)
    pub fn last(&self) -> Option<&T> {
        self.0.map(|node| unsafe {
            let node = node.as_ref();
            node.next.last().unwrap_or(&node.value)
        })
    }

    /// Modify the last value in the linked list.
    /// This is O(n)
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.0.map(|mut node| unsafe {
            let node = node.as_mut();
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
        let node = self.0.take()?;
        let mut node = unsafe { Box::from_raw(node.as_ptr()) };
        match node.next.pop_back() {
            Some(t) => {
                self.0 = Some(Box::leak(node).into());
                Some(t)
            }
            None => {
                Some(node.value)
            }
        }
    }

    /// Create an iter over this linked list
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::from_iter([1, 2, 3]);
    /// assert_eq!(ll.iter().cloned().collect::<Vec<_>>(), vec![1, 2, 3]);
    /// ```
    pub const fn iter(&self) -> Iter<'_, T> {
        Iter(&self.0)
    }

    /// Create a mutable iter over this linked list
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::from_iter([1, 2, 3]);
    /// ll.iter_mut().for_each(|i| *i *= 2);
    /// assert_eq!(ll, LinkedList::from_iter([2, 4, 6]));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(&mut self.0)
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            let node = Box::leak(Box::new(Node::new(v)));
            end.0 = Some(node.into());
            end = &mut node.next;
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T, U> PartialEq<LinkedList<U>> for LinkedList<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &LinkedList<U>) -> bool {
        match (self.0, other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => unsafe {
                a.as_ref().eq(b.as_ref())
            },
            _ => false,
        }
    }
    fn ne(&self, other: &LinkedList<U>) -> bool {
        match (self.0, other.0) {
            (None, None) => false,
            (Some(a), Some(b)) => unsafe {
                a.as_ref().ne(b.as_ref())
            },
            _ => true,
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
    fn ne(&self, other: &Node<U>) -> bool {
        self.value != other.value || self.next != other.next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn push() {
        let mut ll = crate::LinkedList::new();

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
        let mut ll = crate::LinkedList::new();
        assert_eq!(format!("{:?}", ll), "[]");

        ll.extend([1, 2, 3]);

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
}
