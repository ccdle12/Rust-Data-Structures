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
            let result: RefCell<Node<T>> = Rc::try_unwrap(h).ok().expect("");
            result.into_inner().value
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get(&mut self, index: u32) -> Option<T> {
        if index == 0 {
            return self.head();
        }

        let mut current: NodeRef<T> = self.head.clone();
        let mut count = 1;

        while count <= index {
            let mut c = current.as_mut().unwrap();
            let t: &mut RefCell<Node<T>> = &mut *Rc::make_mut(&mut c);
            let result: Node<T> = t.get_mut().clone();
            let r: NodeRef<T> = result.next;
            println!("count: {:?} | result next: {:?}", count, &r);

            //3. Is next None?
            if let Some(_i) = &r {
                //5. is there something at Next?
                //6. Replace a mutable reference which is current
                current = r.clone();
                let mut j = current.as_mut().unwrap();
                let m: &mut RefCell<Node<T>> = &mut *Rc::make_mut(&mut j);
                let b: Node<T> = m.get_mut().clone();
                println!("count: {:?} | new current value: {:?}", count, b);
            } else {
                return None;
            }

            count += 1;
        }

        let mut c = current.as_mut().unwrap();
        let t: &mut RefCell<Node<T>> = &mut *Rc::make_mut(&mut c);
        let result: Node<T> = t.get_mut().clone();

        println!("result value: {:?}", result.value);
        Some(result.value)
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
    /// let head = linked_list.head();
    /// assert_eq!(head, Some("Hello".to_string()));
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
    /// let tail = linked_list.tail();
    /// assert_eq!(tail, Some("World".to_string()));
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

        assert_eq!(linked_list.get(0).unwrap(), "1".to_string());
        assert_eq!(linked_list.get(1).unwrap(), "2".to_string());
        assert_eq!(linked_list.get(2), Some("3".to_string()));
        assert_eq!(linked_list.get(3), Some("4".to_string()));
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
