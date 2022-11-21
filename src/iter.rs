//! A set of iterator types for a [`LinkedList`]

use super::{LinkedList, Node};

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

/// Owned iterator of a [`LinkedList`]
pub struct IntoIter<T>(LinkedList<T>);

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
pub struct Iter<'a, T: 'a>(&'a *const Node<T>);

/// New.
pub fn new_iter<'a, T: 'a>(list: &'a *const Node<T>) -> Iter<'a, T> {
    Iter(list)
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_null() {
            return None;
        }
        let node = self.0;
        unsafe {
            let node = &**node;
            self.0 = &node.next.0;
            Some(&node.value)
        }
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
pub struct IterMut<'a, T: 'a>(&'a mut *const Node<T>);

/// New.
pub fn new_iter_mut<'a, T: 'a>(list: &'a mut *const Node<T>) -> IterMut<'a, T> {
    IterMut(list)
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: 'a,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_null() {
            return None;
        }

        let node = *self.0 as *mut Node<T>;
        unsafe {
            let node = &mut *node;
            self.0 = &mut node.next.0;
            Some(&mut node.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use super::super::LinkedList;

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
