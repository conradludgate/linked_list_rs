#![feature(box_into_inner)]

use std::{fmt::Debug, ops::Deref};

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

    pub fn iter(&self) -> HeadIter<'_, T> {
        HeadIter(self)
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

pub struct HeadIter<'a, T>(&'a Head<T>);
impl<'a, T> Iterator for HeadIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.0.0 {
            None => None,
            Some(node) => {
                let Node { next, value } = node.deref();
                *self = Self(next);
                Some(value)
            },
        }
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
