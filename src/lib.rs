pub mod iter;

use std::{fmt::Debug};

use crate::iter::{Iter, IterMut};

#[derive(Debug, Clone, PartialEq)]
struct Node<T> {
    next: LinkedList<T>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Box<Self> {
        Box::new(Self {
            next: LinkedList::new(),
            value,
        })
    }
}

/// LinkedList is an implementation of a singly-linked-list.
#[derive(Clone, PartialEq)]
pub struct LinkedList<T>(pub(crate) Option<Box<Node<T>>>);

impl<T> LinkedList<T> {
    /// Create a new empty linked list
    pub const fn new() -> Self {
        Self(None)
    }

    /// Get the length of the linked list.
    /// This is an O(n) computation
    pub const fn len(&self) -> usize {
        match &self.0 {
            Some(node) => 1 + node.next.len(),
            None => 0,
        }
    }

    /// Push to the front of the linked list.
    /// This is O(1)
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::new();
    /// ll.push(1);
    /// ll.push(2);
    /// # assert_eq!(ll.len(), 2)
    /// ```
    pub fn push(&mut self, value: T) {
        let next = self.0.replace(Node::new(value));
        if let Some(node) = &mut self.0 {
            node.next = Self(next);
        }
    }

    /// Pop from the front of the linked list.
    /// This is O(1)
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::from_iter([1, 2]);
    /// assert_eq!(ll.pop(), Some(2));
    /// assert_eq!(ll.pop(), Some(1));
    /// assert_eq!(ll.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let Node { next, value } = *self.0.take()?;
        *self = next;
        Some(value)
    }

    /// View the first value in the linked list.
    /// This is O(1)
    pub const fn first(&self) -> Option<&T> {
        match &self.0 {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    /// Modify the first value in the linked list.
    /// This is O(1)
    pub fn first_mut(&mut self) -> Option<&mut T> {
        match &mut self.0 {
            Some(node) => Some(&mut node.value),
            None => None,
        }
    }

    /// View the last value in the linked list.
    /// This is O(n)
    pub const fn last(&self) -> Option<&T> {
        match &self.0 {
            Some(node) => match node.next.last() {
                None => Some(&node.value),
                Some(t) => Some(t),
            },
            None => None,
        }
    }

    /// Modify the last value in the linked list.
    /// This is O(n)
    pub fn last_mut(&mut self) -> Option<&mut T> {
        match &mut self.0 {
            Some(node) => match node.next.last_mut() {
                None => Some(&mut node.value),
                Some(t) => Some(t),
            },
            None => None,
        }
    }

    /// Push to the back of the linked list.
    /// This is O(n)
    pub fn push_back(&mut self, value: T) {
        match &mut self.0 {
            Some(node) => node.next.push_back(value),
            None => self.0 = Some(Node::new(value)),
        }
    }

    /// Pop from the back of the linked list.
    /// This is O(n)
    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.0 {
            Some(node) => match node.next.pop_back() {
                None => self.pop(),
                Some(t) => Some(t),
            },
            None => None,
        }
    }

    /// Create an iter over this linked list
    ///
    /// ```
    /// # use linked::LinkedList;
    /// let mut ll = LinkedList::from_iter([1, 2, 3]);
    /// assert_eq!(ll.iter().cloned().collect::<Vec<_>>(), vec![3, 2, 1]);
    /// ```
    pub const fn iter(&self) -> Iter<'_, T> {
        Iter(self)
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
        IterMut(self)
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
        for v in iter {
            self.push(v)
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self(None)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn push() {
        let mut ll = crate::LinkedList::new();

        ll.push(1);
        ll.push(2);
        ll.push(3);

        assert_eq!(ll.len(), 3);

        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(1));

        assert_eq!(ll.len(), 0);

        assert_eq!(ll.pop(), None);
        assert_eq!(ll.len(), 0);
    }

    #[test]
    fn debug() {
        let mut ll = crate::LinkedList::new();
        assert_eq!(format!("{:?}", ll), "[]");

        ll.extend([1, 2, 3]);

        assert_eq!(format!("{:?}", ll), "[3, 2, 1]");

        assert_eq!(format!("{:#?}", ll), r"[
    3,
    2,
    1,
]");
    }
}
