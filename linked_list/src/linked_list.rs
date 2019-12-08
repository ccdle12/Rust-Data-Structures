use crate::node::{Node, NodeRef};
use std::cell::RefCell;
use std::rc::Rc;

/// LinkedList is a data structure that references each item T in memory, forming
/// a chain of referenced objects.
#[derive(Clone)]
pub struct LinkedList<T> {
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

    /// Returns the head of the List as an Option<T>.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// linked_list.push("Hello".to_string());
    ///
    /// let head = linked_list.head();
    /// assert_eq!(head, Some("Hello".to_string()));
    /// ```
    pub fn head(&self) -> Option<T> {
        self.head.as_ref().map(|h| h.value.clone())
    }

    /// Returns the tail of the List.
    pub fn tail(&self) -> Option<T> {
        self.tail.as_ref().map(|h| h.borrow().value.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_list() {
        let linked_list = LinkedList::<String>::default();
        assert_eq!(linked_list.size, 0);
    }

    #[test]
    fn push_one_node() {
        let mut linked_list = LinkedList::<String>::default();

        linked_list.push("1".to_string());
        assert_eq!(linked_list.size, 1);
    }

    #[test]
    fn push_two_nodes() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..3 {
            linked_list.push(i.to_string());
        }

        assert_eq!(*&linked_list.size, 2);
        assert_eq!(linked_list.head(), Some("1".to_string()));
        assert_eq!(linked_list.tail(), Some("2".to_string()));
    }

    #[test]
    fn push_three_nodes() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..4 {
            linked_list.push(i.to_string());
        }

        assert_eq!(*&linked_list.size, 3);
        assert_eq!(linked_list.tail(), Some("3".to_string()));
    }

    #[test]
    fn access_none_head() {
        let linked_list = LinkedList::<String>::default();
        assert_eq!(linked_list.head(), None);
    }

    #[test]
    fn access_none_tail() {
        let linked_list = LinkedList::<String>::default();
        assert_eq!(linked_list.tail(), None);
    }
}
