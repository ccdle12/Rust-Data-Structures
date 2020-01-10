use crate::node::{Node, NodeRef};
use std::iter::Iterator;

/// LinkedList is a data structure that references each item T in memory, forming
/// a chain of referenced objects.
#[derive(Clone)]
pub struct LinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
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

/// Implements IntoIter for a LinkedList with a lifetime of 'a - the same lifetime
/// as the LinkedList that is being referenced.
impl<'a, T> IntoIterator for &'a LinkedList<T>
where
    T: Clone + std::fmt::Debug,
{
    type Item = T;
    // IntoIter type is a LinkedListIterator of the same lifetime as the LinkedList.
    type IntoIter = LinkedListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator {
            list: self,
            current: None,
        }
    }
}

/// The Iterator implementation for the LinkedList. This Iterator will borrow
/// the LinkedList.
pub struct LinkedListIterator<'a, T> {
    list: &'a LinkedList<T>,
    current: Option<NodeRef<T>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T>
where
    T: Clone + std::fmt::Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match self.current.clone() {
            Some(_) => {
                self.current
                    .clone()
                    .map(|v| self.current = v.0.borrow_mut().next.clone());
            }
            None => {
                self.current = self.list.head.clone();
            }
        };

        self.current.clone().map(|v| v.0.borrow_mut().value.clone())
    }
}

impl<'a, T> DoubleEndedIterator for LinkedListIterator<'a, T>
where
    T: Clone + std::fmt::Debug,
{
    fn next_back(&mut self) -> Option<T> {
        match self.current.clone() {
            Some(_) => {
                self.current
                    .clone()
                    .map(|v| self.current = v.0.borrow_mut().previous.clone());
            }
            None => {
                self.current = self.list.tail.clone();
            }
        };

        self.current.clone().map(|v| v.0.borrow_mut().value.clone())
    }
}

impl<T> LinkedList<T>
where
    T: Clone + std::fmt::Debug,
{
    /// Returns the length of the LinkedList.
    ///
    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    pub fn len(&self) -> usize {
        self.size as usize
    }

    /// Adds a a value to the end of a LinkedList.
    ///
    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    ///
    /// # Example
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
        let new = NodeRef::new(Node::new(v));

        if self.size == 0 {
            self.head = Some(new.clone());
        } else {
            // This works because we take ownership of tail and leave None there.
            // The reason why "old" still exists is because theres another
            // NodeRef pointing to it.
            match self.tail.take() {
                Some(old) => {
                    old.0.borrow_mut().next = Some(new.clone());
                    new.0.borrow_mut().previous = Some(old.clone());
                }
                None => self.head = Some(new.clone()),
            };
        }

        self.tail = Some(new);
        self.size += 1;
    }

    /// Returns the value the head of a LinkedList and removes it from the
    /// LinkedList.
    ///
    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    ///
    /// # Example
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// linked_list.push("Hello".to_string());
    ///
    /// assert_eq!(linked_list.pop_front(), Some("Hello".to_string()));
    /// assert_eq!(linked_list.is_empty(), true);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        // Takes ownership of head.
        // map() applies to the inner value of Option (Rc)
        // map() will return an Option, but we'll change the inner value of it
        // to T.
        self.head.take().map(|mut h| {
            // borrow_mut() - borrows inner value mutably (NodeRef<T>)
            // Takes ownership of next
            //
            // Assign head to next,
            // If there isn't something, head is None, so tail should be None.
            if let Some(next) = h.0.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            // Decrement the size as we have popped from the list.
            self.size -= 1;

            // Extracts the value from h and returns it.
            h.get_value()
        })
    }

    /// Returns the value the tail of a LinkedList and removes it from the
    /// LinkedList.
    ///
    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    ///
    /// # Example
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// linked_list.push("Hello".to_string());
    /// linked_list.push("World".to_string());
    ///
    /// assert_eq!(linked_list.pop_back(), Some("World".to_string()));
    /// assert_eq!(linked_list.len(), 1);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|mut v| {
            if let Some(previous) = v.0.borrow_mut().previous.take() {
                self.tail = Some(previous);
            } else {
                self.head.take();
            }

            self.size -= 1;
            v.get_value()
        })
    }

    /// Returns a boolean indicating the LinkedList is empty.
    ///
    /// # Example
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
    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    ///
    /// # Example
    ///
    /// ```
    /// use linked_list::LinkedList;
    ///
    /// let mut linked_list = LinkedList::<String>::default();
    /// linked_list.push("Hello".to_string());
    ///
    /// assert_eq!(linked_list.get(0), Some("Hello".to_string()));
    /// ```
    pub fn get(&self, index: usize) -> Option<T> {
        let mut current = self.head.clone();

        for _i in 0..index {
            current
                .clone()
                .map(|v| current = v.0.borrow_mut().next.clone());
        }

        current.map(|mut v| v.get_value())
    }

    /// Returns the head of the List as an Option<T>.
    ///
    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    ///
    /// # Example
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
        self.head.as_ref().map(|h| h.0.borrow().value.clone())
    }

    /// Returns the tail of the List.
    ///
    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    ///
    /// # Example
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
        self.tail.as_ref().map(|t| t.0.borrow().value.clone())
    }

    // /// Deletes an item from the list according to an index.
    //
    // /// Time Complexity: O(n)
    // /// Space Complexity: O(1)
    //
    // /// # Example
    // /// ```
    // /// use linked_list::LinkedList;
    // /// let mut linked_list = LinkedList::<String>::default();
    // /// linked_list.push("Hello".to_string());
    // /// linked_list.push("World".to_string());
    //
    // /// linked_list.delete(1);
    // /// assert_eq!(linked_list.len(), 1);
    // /// ```
    // pub fn delete(&mut self, index: u32) -> Result<()> {
    // if index > self.size - 1 {
    //     return Err(LinkedListError::IndexOutOfRangeError);
    // }

    // // Current is the node that will be deleted.
    // // Previous will drop the pointer to current, and then point to the new
    // // next node, that comes after current.
    // let mut previous = self.head.clone();
    // let mut current = previous.clone().unwrap().0.borrow_mut().next.clone();

    // // Delete at head.
    // if index == 0 {
    //     self.head = current.clone();
    //     self.head.clone().map(|v| v.0.borrow_mut().previous = None);
    // }

    // // Deleting greater than head.
    // if index > 0 {
    //     for _i in 0..index - 1 {
    //         previous = current.clone();
    //         current = current.clone().unwrap().0.borrow_mut().next.clone();
    //     }
    // }

    // current.clone().map(|v| v.0.borrow_mut().previous = None);
    // let new_next = current.take().and_then(|v| v.0.borrow_mut().next.clone());
    // previous
    //     .clone()
    //     .map(|v| v.0.borrow_mut().next = new_next.clone());
    // new_next
    //     .clone()
    //     .map(|v| v.0.borrow_mut().previous = previous.clone());

    // self.size -= 1;

    // if self.size == 0 {
    //     self.tail = None;
    //     self.head = None;
    // }

    // if self.size == 1 {
    //     self.tail = self.head.clone();
    //     self.tail.clone().map(|v| v.0.borrow_mut().previous = None);
    //     self.head.clone().map(|v| v.0.borrow_mut().previous = None);
    // }

    // if self.size > 1 {
    //     self.tail = previous;
    // }

    // Ok(())
    // }
}

#[allow(unused_macros)]
macro_rules! linked_list {
    // $ similar to bash script exec
    // execution a variadic number of parameters
    // each expr is seperated by "," for "*" amount of times
    ($($x: expr),*) => {{
        let mut linked_list = LinkedList::default();

        // Push each item to the linked list, according to the number of inputs
        $(linked_list.push($x);)*
        linked_list
    }};
}

#[cfg(test)]
mod singly_linked_list {
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
    fn pop_front() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..5 {
            linked_list.push(i.to_string());
        }

        assert_eq!(linked_list.head(), Some("1".to_string()));
        assert_eq!(linked_list.pop_front(), Some("1".to_string()));
        assert_eq!(linked_list.head(), Some("2".to_string()));
    }

    #[test]
    fn iterator() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..5 {
            linked_list.push(i.to_string());
        }

        let mut iter = linked_list.into_iter();

        // Assert the iterator did not consume the linked_list.
        assert_eq!(linked_list.get(2), Some("3".to_string()));
        assert_eq!(iter.next(), Some("1".to_string()));
        assert_eq!(iter.next(), Some("2".to_string()));
        assert_eq!(iter.next(), Some("3".to_string()));
        assert_eq!(iter.next(), Some("4".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_function_calls() {
        let linked_list = linked_list![1, 2, 3, 4, 5];
        let result: Vec<u32> = linked_list.into_iter().map(|v| (v * 2) as u32).collect();

        assert_eq!(result[0], 2);
        assert_eq!(result[1], 4);
        assert_eq!(result[2], 6);
        assert_eq!(result[3], 8);
        assert_eq!(result[4], 10);
    }

    #[test]
    fn macro_linked_list() {
        let linked_list = linked_list!["1".to_string(), "2".to_string()];
        assert_eq!(linked_list.tail(), Some("2".to_string()));
    }

    #[test]
    fn pop_back() {
        let mut linked_list = linked_list!["1".to_string(), "2".to_string(), "3".to_string()];
        assert_eq!(linked_list.len(), 3);

        let popped = linked_list.pop_back();
        assert_eq!(popped, Some("3".to_string()));
        assert_eq!(linked_list.tail(), Some("2".to_string()));
        assert_eq!(linked_list.len(), 2);

        let popped = linked_list.pop_back();
        assert_eq!(popped, Some("2".to_string()));
        assert_eq!(linked_list.tail(), Some("1".to_string()));
        assert_eq!(linked_list.len(), 1);

        let popped = linked_list.pop_back();
        assert_eq!(popped, Some("1".to_string()));
        assert_eq!(linked_list.tail(), None);
        assert_eq!(linked_list.len(), 0);

        let popped = linked_list.pop_back();
        assert_eq!(popped, None);
        assert_eq!(linked_list.tail(), None);
        assert_eq!(linked_list.len(), 0);
    }
}

#[cfg(test)]
mod doubly_linked_list {
    use super::*;

    #[test]
    fn reverse_iterator() {
        let mut linked_list = LinkedList::<String>::default();

        for i in 1..5 {
            linked_list.push(i.to_string());
        }

        assert_eq!(linked_list.size, 4);
        assert_eq!(linked_list.tail(), Some("4".to_string()));

        let mut iter = linked_list.into_iter();
        assert_eq!(Some("4".to_string()), iter.next_back());
        assert_eq!(Some("3".to_string()), iter.next_back());
        assert_eq!(Some("2".to_string()), iter.next_back());
        assert_eq!(Some("1".to_string()), iter.next_back());
        assert_eq!(None, iter.next_back());
    }

    #[test]
    fn iterator_function_calls() {
        let mut linked_list = linked_list![1, 2, 3, 4, 5];

        let _popped = linked_list.pop_back();
        let _popped = linked_list.pop_front();

        let result: Vec<u32> = linked_list
            .into_iter()
            .rev()
            .map(|v| (v * 2) as u32)
            .collect();

        assert_eq!(result[0], 8);
        assert_eq!(result[1], 6);
        assert_eq!(result[2], 4);
    }
}
