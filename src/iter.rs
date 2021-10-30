use std::iter::FusedIterator;

use crate::{Head, Node};

pub struct IntoIter<T>(pub(crate) Head<T>);

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

pub struct Iter<'a, T>(pub(crate) &'a Head<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Node { next, value } = self.0.0.as_ref().take()?.as_ref();
        self.0 = next;
        Some(value)
    }
}
