use std::cmp::Ordering;

/// Node is a private struct that contains each node in the tree.
#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/// Binary Tree is the main struct holding an adjaceny_list to keep track of
/// nodes in the tree.
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T>
where
    T: Clone + Ord,
{
    pub fn add(&mut self, value: T) {
        // Take ownership of root and move it to the local variable. Root is
        // replaced with None.
        let root = std::mem::replace(&mut self.root, None);

        // add_recursive, rebuilds the entire tree, returning the root.
        self.root = self.add_recursive(root, value);
    }

    fn add_recursive(&mut self, node: Option<Box<Node<T>>>, target: T) -> Option<Box<Node<T>>> {
        match node {
            Some(mut n) => {
                if n.value <= target {
                    n.left = self.add_recursive(n.left, target);
                    Some(n)
                } else {
                    n.right = self.add_recursive(n.right, target);
                    Some(n)
                }
            }
            _ => Some(Box::new(Node {
                value: target,
                left: None,
                right: None,
            })),
        }
    }

    pub fn get(&self, target: T) -> Option<T> {
        self.get_recursive(self.root.clone(), target)
    }

    fn get_recursive(&self, node: Option<Box<Node<T>>>, target: T) -> Option<T> {
        match node {
            Some(n) => match n.value.cmp(&target) {
                Ordering::Less => self.get_recursive(n.left, target),
                Ordering::Greater => self.get_recursive(n.right, target),
                Ordering::Equal => Some(n.value.clone()),
            },
            _ => None,
        }
    }

    fn get_root_node(&self) -> Option<Box<Node<T>>> {
        if self.root.is_some() {
            let node = self.root.clone().unwrap();
            return Some(node);
        }

        None
    }

    pub fn get_root(&self) -> Option<T> {
        if let Some(root) = self.get_root_node() {
            return Some(root.value);
        }

        None
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        BinaryTree { root: None }
    }
}

mod test {
    use super::*;

    #[test]
    fn add_root() {
        let mut btree = BinaryTree::<u16>::default();
        btree.add(10);

        assert_eq!(btree.get_root(), Some(10))
    }

    #[test]
    fn push_and_get() {
        let mut btree = BinaryTree::<u16>::default();
        btree.add(10);
        btree.add(5);
        assert_eq!(btree.get_root(), Some(10));
        assert_eq!(btree.get(5), Some(5));

        btree.add(7);
        assert_eq!(btree.get(7), Some(7));
        assert_eq!(btree.get_root(), Some(10));
    }
}
