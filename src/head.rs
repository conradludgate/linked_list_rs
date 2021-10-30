use std::{fmt::Debug, ops::DerefMut};

use crate::{iter::Iter, node::Node, HeadMut};

#[derive(Clone, PartialEq)]
pub struct Head<T>(pub(crate) Option<Box<Node<T>>>);

impl<T> Head<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn len(&self) -> usize {
        match &self.0 {
            Some(node) => node.len(),
            None => 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let next = self.0.replace(Box::new(Node::new(value)));
        if let Some(node) = &mut self.0 {
            node.next = Self(next);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let Node { next, value } = Box::into_inner(self.0.take()?);
        *self = next;
        Some(value)
    }

    pub fn first(&self) -> Option<&T> {
        match &self.0 {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        match &mut self.0 {
            Some(node) => node.next.push_back(value),
            None => self.0 = Some(Box::new(Node::new(value))),
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.0 {
            Some(node) => match node.next.pop_back() {
                None => self.pop(),
                Some(t) => Some(t),
            },
            None => None,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter(&self)
    }

    pub fn iter_mut(&mut self) -> HeadMut<'_, T> {
        HeadMut(self.0.as_mut().map(DerefMut::deref_mut))
    }
}

impl<T: Debug> Debug for Head<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> FromIterator<T> for Head<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut head = Head::new();
        head.extend(iter);
        head
    }
}

impl<T> Extend<T> for Head<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for v in iter {
            self.push(v)
        }
    }
}