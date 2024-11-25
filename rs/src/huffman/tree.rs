use std::cmp::Ordering;

/// A node in the binary tree.
#[derive(Debug)]
pub struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
pub struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
  pub fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    pub  fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }
}

impl<T: Ord> Subtree<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub  fn insert(&mut self, value: T) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(value))),
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.insert(value),
                Ordering::Equal => {}
                Ordering::Greater => n.right.insert(value),
            },
        }
    }

    pub   fn has(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.has(value),
                Ordering::Equal => true,
                Ordering::Greater => n.right.has(value),
            },
        }
    }

    pub   fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(n) => 1 + n.left.len() + n.right.len(),
        }
    }
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}

// fn main() {
//     let mut tree = BinaryTree::new();
//     tree.insert("foo");
//     assert_eq!(tree.len(), 1);
//     tree.insert("bar");
//     assert!(tree.has(&"foo"));
// }
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
pub struct Leaf {
    pub(crate) ch: char,
    pub(crate) frequency: i32,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree: BinaryTree<Leaf> = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        let leaf = Leaf {
            frequency: 33,
            ch: 's',
        };
        tree.insert(leaf);
        assert_eq!(tree.len(), 1);
        // tree.insert(1);
        // assert_eq!(tree.len(), 2);
        // tree.insert(2); // not a unique item
        // assert_eq!(tree.len(), 2);
    }
    //
    // #[test]
    // fn has() {
    //     let mut tree = BinaryTree::new();
    //     fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
    //         let got: Vec<bool> =
    //             (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
    //         assert_eq!(&got, exp);
    //     }
    //
    //     check_has(&tree, &[false, false, false, false, false]);
    //     tree.insert(0);
    //     check_has(&tree, &[true, false, false, false, false]);
    //     tree.insert(4);
    //     check_has(&tree, &[true, false, false, false, true]);
    //     tree.insert(4);
    //     check_has(&tree, &[true, false, false, false, true]);
    //     tree.insert(3);
    //     check_has(&tree, &[true, false, false, true, true]);
    // }
    //
    // #[test]
    // fn unbalanced() {
    //     let mut tree = BinaryTree::new();
    //     for i in 0..100 {
    //         tree.insert(i);
    //     }
    //     assert_eq!(tree.len(), 100);
    //     assert!(tree.has(&50));
    // }
}
