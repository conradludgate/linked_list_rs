use std::ops::DerefMut;

use crate::Node;


pub struct HeadMut<'a, T>(pub(crate) Option<&'a mut Node<T>>);

impl<'a, T> HeadMut<'a, T> {
    fn pop(&mut self) -> Option<&'a mut T> {
        let Node { next, value } = self.0.take()?;
        *self = Self(next.0.as_mut().map(DerefMut::deref_mut));
        Some(value)
    }
}

impl<'a, T> Iterator for HeadMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
