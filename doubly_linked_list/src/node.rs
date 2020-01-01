use std::cell::RefCell;
use std::rc::Rc;

/// Alias for a referenced Node.
#[derive(Debug, Clone)]
pub(crate) struct NodeRef<T>(pub Rc<RefCell<Node<T>>>);

impl<T> NodeRef<T>
where
    T: Clone,
{
    /// new returns a NodeRef that wraps a Node<T> in a Rc<RefCell<Node<T>>.
    pub fn new(v: Node<T>) -> NodeRef<T> {
        NodeRef(Rc::new(RefCell::new(v)))
    }

    /// Clones the value in the NodeRef.
    pub fn get_value(&mut self) -> T {
        self.0.borrow_mut().value.clone()
    }
}

/// Node is the structure in a LinkedList. It contains a pointer to the next
/// Node in memory and holds a value `T`.
#[derive(Debug, Clone)]
pub(crate) struct Node<T> {
    pub value: T,
    pub next: Option<NodeRef<T>>,
    pub previous: Option<NodeRef<T>>,
}

#[allow(dead_code)]
impl<T> Node<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            next: None,
            previous: None,
        }
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
        // head.set_next(next);
        head.next = next;

        assert_eq!(
            head.next.unwrap().0.try_borrow().unwrap().value,
            "world".to_string()
        );
    }

    #[test]
    fn previous_node() {
        let mut head = Node::new(1);
        let mut mid = Node::new(2);
        let mut tail = Node::new(3);

        head.next = Some(NodeRef::new(mid.clone()));
        mid.previous = Some(NodeRef::new(head.clone()));
        mid.next = Some(NodeRef::new(tail.clone()));
        tail.previous = Some(NodeRef::new(mid.clone()));

        assert_eq!(mid.previous.unwrap().get_value(), 1);
        assert_eq!(mid.next.unwrap().get_value(), 3);
    }
}
