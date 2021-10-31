use std::iter::FusedIterator;

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
        let Node { next, value } = self.0 .0.as_ref().take()?.as_ref();
        self.0 = next;
        Some(value)
    }
}

pub struct IterMut<'a, T>(pub(crate) &'a mut LinkedList<T>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // Safety: self is a mut reference, therefore it must be valid
        // The pointer is not staying beyond the scope of this function
        // The *mut pointer is used to bypass some awkward lifetime rules
        // where the 'b lifetime takes precendence over the 'a lifetime

        let ll = std::ptr::addr_of_mut!(self.0);

        unsafe {
            let Node { next, value } = (*ll).0.as_mut().take()?.as_mut();
            *ll = next;
            Some(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn iter() {
        let ll = LinkedList::from_iter([1, 2, 3]);

        assert_eq!(ll.iter().cloned().collect::<Vec<_>>(), vec![3, 2, 1]);
    }

    #[test]
    fn iter_mut() {
        let mut ll = LinkedList::from_iter([1, 2, 3]);
        ll.iter_mut().for_each(|i| *i *= 2);

        assert_eq!(ll, LinkedList::from_iter([2, 4, 6]));
    }
}
