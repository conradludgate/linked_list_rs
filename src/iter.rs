//! A set of iterator types for a [`LinkedList`]

use std::{iter::FusedIterator, ptr::NonNull};

use crate::{LinkedList, Node};

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

/// Owned iterator of a [`LinkedList`]
pub struct IntoIter<T>(pub(crate) LinkedList<T>);

impl<T> FusedIterator for IntoIter<T> {}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        Iter(&self.0)
    }
}

/// Borrowed iterator of a [`LinkedList`]
#[derive(Clone)]
pub struct Iter<'a, T>(pub(crate) &'a Option<NonNull<Node<T>>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| unsafe {
            let node = &*node.as_ptr();
            self.0 = &node.next.0;
            &node.value
        })
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        IterMut(&mut self.0)
    }
}

/// Mutable iterator of a [`LinkedList`]
pub struct IterMut<'a, T>(pub(crate) &'a mut Option<NonNull<Node<T>>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| unsafe {
            let node = &mut *node.as_ptr();
            self.0 = &mut node.next.0;
            &mut node.value
        })
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::LinkedList;

    #[test]
    fn iter() {
        let ll = LinkedList::from_iter(vec![1, 2, 3]);

        assert_eq!(ll.iter().cloned().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn iter_mut() {
        let mut ll = LinkedList::from_iter(vec![1, 2, 3]);
        ll.iter_mut().for_each(|i| *i *= 2);

        assert_eq!(ll.into_iter().collect::<Vec<_>>(), [2, 4, 6]);
    }
}
