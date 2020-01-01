use std::cell::RefCell;
use std::rc::Rc;

/// Alias for a referenced Node.
#[derive(Debug, Clone)]
pub(crate) struct NodeRef<T>(pub Rc<RefCell<Node<T>>>);

impl<T> NodeRef<T> {
    pub fn new(v: Node<T>) -> NodeRef<T> {
        NodeRef(Rc::new(RefCell::new(v)))
    }

    pub fn extract_value(self) -> T {
        // try_unwrap(h) will return the value in a Result if it has exactly
        // ONLY one reference.
        //
        // ok() if there is something in there, and returns as an Option<T>.
        // expect returns the value if Some
        //
        // into_inner() - returns the value inside of the Option<T>
        Rc::try_unwrap(self.0)
            .ok()
            .expect("something went wrong")
            .into_inner()
            .value
    }
}

/// Node is the structure in a LinkedList. It contains a pointer to the next
/// Node in memory and holds a value `T`.
#[derive(Debug, Clone)]
pub(crate) struct Node<T> {
    pub value: T,
    pub next: Option<NodeRef<T>>,
}

#[allow(dead_code)]
impl<T> Node<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }

    pub fn set_next(&mut self, next: Option<NodeRef<T>>) {
        self.next = next;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_node() {
        let node = Node::new(5);
        assert_eq!(node.value, 5);
    }

    #[test]
    fn mutate_value() {
        let mut head = Node::new("hello".to_string());

        head.value = "!".to_string();
        assert_eq!(head.value, "!".to_string());
    }

    #[test]
    fn next_node() {
        let mut head = Node::new("hello".to_string());
        let tail = Node::new("world".to_string());

        let next = Some(NodeRef::new(tail));
        head.set_next(next);

        assert_eq!(
            head.next.unwrap().0.try_borrow().unwrap().value,
            "world".to_string()
        );
    }
}
