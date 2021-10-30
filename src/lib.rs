#![feature(box_into_inner)]

mod node;
mod head;
mod head_ref;
mod head_mut;

pub use node::Node;
pub use head::Head;
pub use head_ref::HeadRef;
pub use head_mut::HeadMut;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let mut head = crate::Head::new();

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
}
