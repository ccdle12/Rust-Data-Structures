use crate::node::{Node, NodeRef};
use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;

/// LinkedList is a data structure that references each item T in memory, forming
/// a chain of referenced objects.
#[derive(Clone)]
pub struct LinkedList<T> {
    head: NodeRef<T>,
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
    T: Clone + std::fmt::Debug,
{
    /// Adds a a value to the end of a LinkedList.
    ///
    /// # Examples
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// linked_list.push("Hello".to_string());
    ///
    /// assert_eq!(linked_list.tail(), Some("Hello".to_string()));
    /// ```
    pub fn push(&mut self, v: T) {
        let new = Rc::new(RefCell::new(Node::new(v)));

        // This works because we take ownership of tail and leave None there.
        // The reason why "old" still exists is because theres another
        // NodeRef pointing to it.
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };

        self.size += 1;
        self.tail = Some(new);
    }

    /// Returns the value from a LinkedList and removes it from the LinkedList.
    ///
    /// # Examples
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// linked_list.push("Hello".to_string());
    ///
    /// assert_eq!(linked_list.pop(), Some("Hello".to_string()));
    /// assert_eq!(linked_list.is_empty(), true);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        // Takes ownership of head.
        // map() applies to the inner value of Option (Rc)
        // map() will return an Option, but we'll change the inner value of it
        // to T.
        self.head.take().map(|h| {
            // borrow_mut() - borrows inner value mutably (NodeRef<T>)
            // Takes ownership of next
            //
            // Assign head to next,
            // If there isn't something, head is None, so tail should be None.
            if let Some(next) = h.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            // Decrement the size as we have popped from the list.
            self.size -= 1;

            // try_unwrap(h) will return the value in a Result if it has exactly
            // ONLY one reference.
            // Ok if there is something in there, and returns as an Option<T>.
            // expect returns the value if Some
            let result: RefCell<Node<T>> = Rc::try_unwrap(h).ok().expect("something went wrong");
            result.into_inner().value
        })
    }

    /// Returns a boolean if a LinkedList is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// assert_eq!(linked_list.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Gets the value from a LinkedList according to an index.
    ///
    /// # Examples
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// linked_list.push("Hello".to_string());
    ///
    /// assert_eq!(linked_list.get(0), Some("Hello".to_string()));
    /// ```
    pub fn get(&mut self, index: u32) -> Option<T> {
        let mut current = self.head.clone();

        for _i in 0..index {
            current.clone().map(|v| match v.borrow_mut().next.clone() {
                Some(n) => current = Some(n),
                None => current = None,
            });
        }

        current.map(|v| v.borrow_mut().value.clone())
    }

    /// Returns the head of the List as an Option<T>.
    ///
    /// # Examples
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// linked_list.push("Hello".to_string());
    ///
    /// assert_eq!(linked_list.head(), Some("Hello".to_string()));
    /// ```
    pub fn head(&self) -> Option<T> {
        self.head.as_ref().map(|h| h.borrow().value.clone())
    }

    /// Returns the tail of the List.
    ///
    /// # Examples
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// linked_list.push("Hello".to_string());
    /// linked_list.push("World".to_string());
    ///
    /// assert_eq!(linked_list.tail(), Some("World".to_string()));
    /// ```
    pub fn tail(&self) -> Option<T> {
        self.tail.as_ref().map(|t| t.borrow().value.clone())
    }
}

// TODO(ccdle12): Seems like I need to create a wrapper IterStruct around the
// LinkedList.
// Implementations seem to just call pop()
impl<T> Iterator for LinkedList<T>
where
    T: Clone + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
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

    #[test]
    fn head_and_tail() {
        let mut linked_list = LinkedList::<String>::default();

        linked_list.push(String::from("hello"));
        assert_eq!(linked_list.head(), Some("hello".to_string()));
        assert_eq!(linked_list.tail(), Some("hello".to_string()));
        assert_eq!(linked_list.size, 1);

        linked_list.push("world".to_string());
        assert_eq!(linked_list.tail(), Some("world".to_string()));
        assert_eq!(linked_list.size, 2);
    }

    #[test]
    fn get_at_index() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..5 {
            linked_list.push(i.to_string());
        }
        assert_eq!(linked_list.head(), Some("1".to_string()));
        assert_eq!(linked_list.tail(), Some("4".to_string()));

        assert_eq!(linked_list.get(0), Some("1".to_string()));
        assert_eq!(linked_list.get(1), Some("2".to_string()));
        assert_eq!(linked_list.get(2), Some("3".to_string()));
        assert_eq!(linked_list.get(3), Some("4".to_string()));
        assert_eq!(linked_list.get(4), None);
        assert_eq!(linked_list.get(100), None);
    }

    #[test]
    fn pop() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..5 {
            linked_list.push(i.to_string());
        }

        assert_eq!(linked_list.head(), Some("1".to_string()));
        assert_eq!(linked_list.pop(), Some("1".to_string()));
        assert_eq!(linked_list.head(), Some("2".to_string()));
    }

    #[test]
    fn iterator() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..5 {
            linked_list.push(i.to_string());
        }

        for i in linked_list.into_iter() {
            assert_eq!(i, format!("{}", i));
        }
    }
}
