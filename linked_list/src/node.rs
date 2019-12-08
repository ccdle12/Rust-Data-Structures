use std::cell::RefCell;
use std::rc::Rc;

/// Alias for a referenced Node.
pub(crate) type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;

/// Node is the structure in a LinkedList. It contains a pointer to the next
/// Node in memory and holds a value `T`.
#[derive(Debug, Clone)]
pub(crate) struct Node<T> {
    pub value: T,
    pub next: NodeRef<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }

    pub fn set_next(&mut self, next: NodeRef<T>) {
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

        let n = Some(Rc::new(RefCell::new(tail)));
        head.set_next(n);

        assert_eq!(
            head.next.unwrap().try_borrow().unwrap().value,
            "world".to_string()
        );
    }
}
