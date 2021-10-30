use crate::LinkedList;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    pub(crate) next: LinkedList<T>,
    pub(crate) value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            next: LinkedList::new(),
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
