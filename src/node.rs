use crate::LinkedList;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    pub(crate) next: LinkedList<T>,
    pub(crate) value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Box<Self> {
        Box::new(Self {
            next: LinkedList::new(),
            value,
        })
    }
}
