use std::cell::RefCell;
use std::rc::Rc;

/// Alias for a referenced Node.
type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;

/// Node is the structure in a LinkedList. It contains a pointer to the next
/// Node in memory and holds a value `T`.
#[derive(Debug, Clone)]
struct Node<T> {
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

#[derive(Clone)]
struct LinkedList<T> {
    head: Option<Node<T>>,
    tail: NodeRef<T>,
    size: u32,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }
}

#[allow(dead_code)]
impl<T> LinkedList<T>
where
    T: Clone,
{
    pub fn push(&mut self, v: T) {
        if self.is_empty() {
            self.head = Some(Node::new(v.clone()));
        }

        if self.only_head() {
            self.tail = Some(Rc::new(RefCell::new(Node::new(v.clone()))));
            self.head
                .as_mut()
                .unwrap()
                .set_next(Some(self.tail.as_ref().unwrap().clone()));
        }

        // this literally makes me sick
        if !self.is_empty() && !self.only_head() {
            // as_mut() - gets the value in the Option as a mutable reference.
            // &Option<T> -> Option<&mut T>
            let mut tail = self.tail.as_mut().unwrap();

            // &mut *Rc::make_mut(&mut tail) - Gets a mutable reference to the
            // inner value of Rc<> which is RefCell.
            let t: &mut RefCell<Node<T>> = &mut *Rc::make_mut(&mut tail);

            // RefCell.get_mut() - Gets a mutable reference to the inner data.
            let inner: &mut Node<T> = t.get_mut();

            // Clone the v and wrap it as a NodeRef.
            let next = Rc::new(RefCell::new(Node::new(v.clone())));

            // Sets the previous tail pointer to the next new tail.
            inner.set_next(Some(next.clone()));

            // Linked List updates the tail reference.
            self.tail = Some(next.clone());
        }

        self.size += 1;
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn only_head(&self) -> bool {
        self.size == 1
    }

    pub fn tail(&self) -> T {
        let tail = self.tail.as_ref().unwrap().clone();
        let result = tail.borrow().value.clone();

        result
    }

    pub fn head(&self) -> T {
        let head = self.head.as_ref().unwrap().clone();
        let result = head.value.clone();

        result
    }
}

#[cfg(test)]
mod node_tests {
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

#[cfg(test)]
mod linked_list_tests {
    use super::*;

    #[test]
    fn init_list() {
        let linked_list = LinkedList::<String>::default();
        assert_eq!(linked_list.size, 0);
    }

    #[test]
    fn push_one_node() {
        let mut linked_list = LinkedList::<String>::default();

        let a = "1".to_string();
        linked_list.push(a);

        assert_eq!(linked_list.size, 1);
    }

    #[test]
    fn push_two_nodes() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..3 {
            linked_list.push(i.to_string());
        }
        assert_eq!(*&linked_list.size, 2);

        let head = linked_list.head();
        assert_eq!(head, "1".to_string());

        let tail = linked_list.tail();
        assert_eq!(tail, "2".to_string());
    }

    #[test]
    fn push_three_nodes() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..4 {
            linked_list.push(i.to_string());
        }
        assert_eq!(*&linked_list.size, 3);

        let tail = linked_list.tail();
        assert_eq!(tail, "3".to_string());
    }
}
