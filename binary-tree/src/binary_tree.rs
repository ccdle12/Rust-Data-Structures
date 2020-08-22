/// Binary Tree is the main struct holding an adjaceny_list to keep track of
/// nodes in the tree.
pub struct BinaryTree<T> {
    adjaceny_list: Vec<Node<T>>,
}

impl<T> BinaryTree<T>
where
    T: Clone + Ord,
{
    // TODO:
    // 1. Refactor Option<bool> -> Result<Option<bool>>
    pub fn push(&mut self, value: T) -> Option<bool> {
        // 1. if 0 index is empty, add it so it is root.
        if self.adjaceny_list.len() == 0 {
            let node = Node {
                value: value.clone(),
                index: 0,
                left: 1,
                right: 2,
            };

            self.adjaceny_list.insert(0, node);
        }

        // 2. Traverse and add a node.
        // - While left is not empty?
        // if value < self.get_root().unwrap() {

        let root = Some(self.get_root_node().unwrap().clone());
        self.recursive_push(value, root);

        Some(true)
    }

    // TODO: Return a Result<>
    // - Figure out lifetime annotations.
    fn recursive_push(&mut self, target: T, current: Option<Node<T>>) -> Option<bool> {
        let current = current.unwrap();

        if target < current.value {
            return match self.get_at_index(current.left) {
                Some(x) => self.recursive_push(target, Some(x.clone())),
                None => {
                    let node = Node {
                        value: target.clone(),
                        index: current.left,
                        left: (current.left + 2),
                        right: (current.left + 3),
                    };

                    self.adjaceny_list.insert(current.left.into(), node);

                    Some(true)
                }
            };
        }

        Some(true)
    }

    /// Returns a target T from the BinaryTree.
    pub fn get<'a>(&'a self, target: &T) -> Option<&'a T> {
        // TODO: Handle unwrap
        // - Replace with ? using a error/result type for the project.
        // - If an Err, return empty list error.
        let current = self.get_root_node().unwrap();

        match self.recursive_get(target, &current) {
            Some(x) => Some(&x.value),
            None => None,
        }
    }

    /// Internal function. It recursively walks the three until the target T is
    /// found.
    fn recursive_get<'a>(&'a self, target: &T, current: &'a Node<T>) -> Option<&'a Node<T>> {
        if current.value == *target {
            return Some(current);
        }

        if *target < current.value {
            return match self.get_at_index(current.left) {
                Some(next_node) => self.recursive_get(target, next_node),
                None => None,
            };
        }

        None
    }

    fn get_at_index(&self, index: u16) -> Option<&Node<T>> {
        self.adjaceny_list.get(index as usize)
    }

    fn get_root_node(&self) -> Option<&Node<T>> {
        self.adjaceny_list.get(0)
    }

    pub fn get_root(&self) -> Option<&T> {
        Some(&self.get_root_node().unwrap().value)
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        BinaryTree {
            adjaceny_list: Vec::new(),
        }
    }
}

/// Node is a private struct that contains each node in the tree.
#[derive(Clone)]
struct Node<T> {
    value: T,
    index: u16,
    left: u16,
    right: u16,
}

mod test {
    use super::*;

    #[test]
    fn add_root() {
        let mut btree = BinaryTree::<u16>::default();
        btree.push(10);
        assert_eq!(btree.get_root(), Some(&10))
    }

    #[test]
    fn push_and_get() {
        let mut btree = BinaryTree::<u16>::default();
        btree.push(10);
        btree.push(5);
        assert_eq!(btree.get_root(), Some(&10));
        assert_eq!(btree.get(&5), Some(&5))
    }
}
