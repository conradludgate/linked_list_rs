use std::ops::Deref;

use crate::Node;


pub struct HeadRef<'a, T>(pub(crate) Option<&'a Node<T>>);

impl<'a, T> HeadRef<'a, T> {
    fn pop(&mut self) -> Option<&'a T> {
        let Node { next, value } = self.0.take()?;
        *self = Self(next.0.as_ref().map(Deref::deref));
        Some(value)
    }
}

impl<'a, T> Iterator for HeadRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
