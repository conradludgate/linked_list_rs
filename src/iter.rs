use std::{iter::FusedIterator, ops::DerefMut};

use crate::{LinkedList, Node};

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(pub(crate) LinkedList<T>);

impl<T> FusedIterator for IntoIter<T> {}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
    }
}

pub struct Iter<'a, T>(pub(crate) &'a LinkedList<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Node { next, value } = self.0.0.as_ref().take()?.as_ref();
        self.0 = next;
        Some(value)
    }
}

pub struct IterMut<'a, T>(pub(crate) Option<&'a mut Node<T>>);

impl<'a, T> IterMut<'a, T> {
    fn pop(&mut self) -> Option<&'a mut T> {
        let Node { next, value } = self.0.take()?;
        *self = Self(next.0.as_mut().map(DerefMut::deref_mut));
        Some(value)
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
