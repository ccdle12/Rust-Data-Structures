/// LRU - Least Recently Used Cache
///
/// Guarantees:
/// - Read: O(1)
/// - Write: O(1)
/// - Eviction: O(1)
///
/// Abstract Datastructure:
/// - read(T)
///     - Look up MAP, follow ptr to get value in LIST
/// - write(T)
///     - When we add to the LRU we:
///         - Check if it's in the HashMap, if cache miss:
///             - Add item to HEAD of list
///             - Add item in MAP with ptr to list
///         - If adding new item of LRU will be greater than size limit then evict()
///             - Then add new item
///
/// - (private) evict()
///     - Look up TAIL in list:
///         - remove previous pointer
///         - remove tail pointer and give to previous
///         - drop from memory
///         - remove item in HashMap
///
/// Datastructure:
/// - LinkedList (Doubly):
///     - Contains: T: the key, V: some interesting value
///
/// - HashMap:
///     - Contains: T (key), V (ptr)
///
/// Invariants:
/// - size of LRU
///
/// LinkedList ADT:
/// - insert_head()
/// - remove() // assumes only removing from tail
///
use std::{cell::RefCell, cmp::PartialEq, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Clone)]
struct Node<K: Clone + PartialEq, V: Clone> {
    pub value: (K, V),
    pub next: Option<NodeRef<K, V>>,
    pub prev: Option<NodeRef<K, V>>,
}

#[derive(Clone)]
struct NodeRef<K: Clone + PartialEq, V: Clone>(pub Rc<RefCell<Node<K, V>>>);

impl<K: Clone + PartialEq, V: Clone> NodeRef<K, V> {
    pub fn init(key: K, value: V) -> NodeRef<K, V> {
        let node = Node {
            value: (key, value),
            next: None,
            prev: None,
        };

        NodeRef(Rc::new(RefCell::new(node)))
    }

    pub fn get_value(&self) -> (K, V) {
        self.0.borrow().value.clone()
    }

    pub fn get_next(&self) -> Option<NodeRef<K, V>> {
        self.0.borrow().next.clone()
    }
}

struct DoublyLinkedList<K: Clone + PartialEq, V: Clone> {
    pub head: Option<NodeRef<K, V>>,
    pub tail: Option<NodeRef<K, V>>,
    pub size: usize,
}

impl<K: Clone + PartialEq, V: Clone> DoublyLinkedList<K, V> {
    pub fn init() -> DoublyLinkedList<K, V> {
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn get_head(&self) -> Option<NodeRef<K, V>> {
        self.head.clone()
    }

    pub fn get_tail(&self) -> Option<NodeRef<K, V>> {
        self.tail.clone()
    }

    pub fn insert(&mut self, value: (K, V)) {
        self.insert_node(NodeRef::init(value.0, value.1), true);
    }

    pub fn insert_node(&mut self, new_head: NodeRef<K, V>, new_node: bool) {
        match self.head.take() {
            Some(prev) => {
                prev.0.borrow_mut().prev = Some(new_head.clone());
                new_head.0.borrow_mut().next = Some(prev.clone());

                if self.size == 1 {
                    self.tail = Some(prev.clone());
                }
            }
            None => (),
        }

        self.head = Some(new_head.clone());

        if new_node {
            self.size += 1;
        }
    }

    pub fn requeue_node(&mut self, node: NodeRef<K, V>) {
        let prev_node = node.0.borrow_mut().prev.clone();
        let next_node = node.0.borrow_mut().next.clone();

        match prev_node.clone() {
            Some(p) => p.0.borrow_mut().next = next_node.clone(),
            _ => (),
        }

        node.0.borrow_mut().prev = None;
        node.0.borrow_mut().next = None;

        match next_node {
            Some(n) => n.0.borrow_mut().prev = prev_node.clone(),
            _ => (),
        }

        match self.get_tail() {
            Some(t) => {
                let tail_key = t.0.borrow().value.0.clone();
                let node_key = node.0.borrow().value.0.clone();

                if tail_key == node_key {
                    self.tail = prev_node.clone();
                }
            }
            _ => (),
        }

        self.insert_node(node, false);
    }

    pub fn remove(&mut self) {
        match self.tail.take() {
            Some(old_tail) => {
                let new_tail = old_tail.0.borrow_mut().prev.clone();

                if let Some(t) = new_tail.clone() {
                    t.clone().0.borrow_mut().next = None;
                }
                old_tail.0.borrow_mut().prev = None;

                self.tail = new_tail.clone();
                self.size -= 1;

                if self.size == 0 {
                    self.head = None;
                }
            }
            _ => (),
        }
    }
}

struct LRU<K: Clone + PartialEq, V: Clone> {
    list: DoublyLinkedList<K, V>,
    map: HashMap<K, NodeRef<K, V>>,
    limit: usize,
    size: usize,
}

impl<K: Clone + Eq + Hash, V: Clone> LRU<K, V> {
    pub fn init(limit: usize) -> LRU<K, V> {
        LRU {
            list: DoublyLinkedList::init(),
            map: HashMap::new(),
            limit,
            size: 0,
        }
    }

    pub fn add(&mut self, key: K, value: V) {
        let node = NodeRef::init(key.clone(), value.clone());

        if self.size == self.limit {
            match self.list.get_tail() {
                Some(t) => {
                    let key = &t.0.borrow().value.0;
                    self.map.remove(&key);
                }
                None => (),
            }

            self.list.remove();
            self.size -= 1;
        }

        match self.map.insert(key, node.clone()) {
            Some(_) => return,
            None => (),
        }
        self.list.insert_node(node, true);
        self.size += 1;
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        match self.map.get(&key) {
            Some(node) => {
                let item = node.clone();
                self.list.requeue_node(item.clone());

                let value = Some(item.0.borrow().value.1.clone());
                value
            }
            _ => None,
        }
    }
}

fn main() {}

mod test {
    use super::*;

    #[test]
    fn init_node() {
        let node = NodeRef::init("hello".to_string(), 0);
        assert_eq!(node.get_value(), ("hello".to_owned(), 0));
    }

    #[test]
    fn init_list() {
        let mut list = DoublyLinkedList::<String, u8>::init();

        list.insert(("APPLE".to_owned(), 30));
        list.insert(("GOOGLE".to_owned(), 50));

        assert_eq!(list.get_head().unwrap().get_value().0, "GOOGLE".to_owned());
        assert_eq!(
            list.get_head().unwrap().get_next().unwrap().get_value().0,
            "APPLE".to_owned()
        );
        assert_eq!(list.size, 2);

        list.insert(("FACEBOOK".to_owned(), 100));
        assert_eq!(list.size, 3);
        assert_eq!(
            list.get_head().unwrap().get_value().0,
            "FACEBOOK".to_owned()
        );

        assert_eq!(list.get_tail().unwrap().get_value().0, "APPLE".to_owned());
        assert_eq!(
            list.get_head().unwrap().get_value().0,
            "FACEBOOK".to_owned()
        );
        let next = list.get_head().unwrap().get_next();
        assert_eq!(next.as_ref().unwrap().get_value().0, "GOOGLE".to_owned());
        assert_eq!(
            next.as_ref().unwrap().get_next().unwrap().get_value().0,
            "APPLE".to_owned()
        );

        list.remove();
        assert_eq!(list.size, 2);
        assert_eq!(
            list.get_head().unwrap().get_value().0,
            "FACEBOOK".to_owned()
        );
        assert_eq!(list.get_tail().unwrap().get_value().0, "GOOGLE".to_owned());
        assert!(list.get_tail().unwrap().get_next().is_none());

        list.remove();
        assert_eq!(list.size, 1);
        assert_eq!(
            list.get_head().unwrap().get_value().0,
            "FACEBOOK".to_owned()
        );
        assert_eq!(
            list.get_tail().unwrap().get_value().0,
            "FACEBOOK".to_owned()
        );
        assert!(list.get_tail().unwrap().get_next().is_none());

        list.remove();
        assert_eq!(list.size, 0);
        assert!(list.get_head().is_none());
        assert!(list.get_tail().is_none());
    }

    #[test]
    fn init_lru() {
        let mut lru = LRU::<String, u32>::init(4);
        lru.add("GOOGLE".to_string(), 50);
        lru.add("FACEBOOK".to_string(), 100);
        lru.add("APPLE".to_string(), 20);
        lru.add("AMAZON".to_string(), 20);
        lru.add("QUALCOMM".to_string(), 20);

        assert_eq!(lru.size, 4);

        // GOOGLE should have been evicted
        assert!(lru.get("GOOGLE".to_string()).is_none());

        assert_eq!(lru.get("FACEBOOK".to_string()).unwrap(), 100);
        assert_eq!(lru.get("APPLE".to_string()).unwrap(), 20);
        assert_eq!(lru.get("AMAZON".to_string()).unwrap(), 20);
        assert_eq!(lru.get("QUALCOMM".to_string()).unwrap(), 20);
        assert_eq!(lru.get("FACEBOOK".to_string()).unwrap(), 100);

        lru.add("NVIDIA".to_string(), 20);
        assert!(lru.get("APPLE".to_string()).is_none());
    }
}
