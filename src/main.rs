#![feature(box_into_inner)]

use std::{fmt::Debug, ops::{Deref, DerefMut}};

pub struct Head<T>(Option<Box<Node<T>>>);

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

    pub fn push_back(&mut self, value: T) {
        match &mut self.0 {
            Some(node) => node.next.push_back(value),
            None => self.0 = Some(Box::new(Node::new(value)))
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.0 {
            Some(node) => {
                match node.next.pop_back() {
                    None => self.pop(),
                    Some(t) => Some(t),
                }
            },
            None => None,
        }
    }

    pub fn iter(&self) -> HeadIter<'_, T> {
        HeadIter(self.0.as_ref().map(Deref::deref))
    }

    pub fn iter_mut(&mut self) -> HeadIterMut<'_, T> {
        HeadIterMut(self.0.as_mut().map(DerefMut::deref_mut))
    }
}

impl<T: Debug> Debug for Head<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Iterator for Head<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

pub struct HeadIter<'a, T>(Option<&'a Node<T>>);
impl<'a, T> Iterator for HeadIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Node { next, value } = self.0.take()?;
        *self = Self(next.0.as_ref().map(Deref::deref));
        Some(value)
    }
}

pub struct HeadIterMut<'a, T>(Option<&'a mut Node<T>>);
impl<'a, T> Iterator for HeadIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let Node { next, value } = self.0.take()?;
        *self = Self(next.0.as_mut().map(DerefMut::deref_mut));
        Some(value)
    }
}

#[derive(Debug)]
pub struct Node<T> {
    next: Head<T>,
    value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            next: Head::new(),
            value,
        }
    }

    pub fn len(&self) -> usize {
        1 + self.next.len()
    }
}

impl<T> std::ops::Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let mut head = Head::new();

    head.push(1);
    head.push(2);
    head.push(3);

    assert_eq!(head.len(), 3);

    println!("{:?}", head);

    assert_eq!(head.pop(), Some(3));
    assert_eq!(head.pop(), Some(2));
    assert_eq!(head.pop(), Some(1));

    assert_eq!(head.len(), 0);

    assert_eq!(head.pop(), None);
    assert_eq!(head.len(), 0);

    println!("{:?}", head);
}
