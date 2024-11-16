use std::cmp::Ordering;

pub trait HuffBaseNode {
    fn is_leaf_node(&self) -> bool;
    fn weight(&self) -> usize;
}

#[derive(PartialEq)]
struct HuffLeafNode {
    element: char,
    weight: usize,
}

impl HuffBaseNode for HuffLeafNode {
    fn is_leaf_node(&self) -> bool {
        true
    }

    fn weight(&self) -> usize {
        self.weight
    }
}

struct HuffInternalNode<'a> {
    weight: usize,
    left: &'a dyn HuffBaseNode,
    right: &'a dyn HuffBaseNode,
}

impl HuffBaseNode for HuffInternalNode<'_> {
    fn is_leaf_node(&self) -> bool {
        false
    }

    fn weight(&self) -> usize {
        self.weight
    }
}

struct HuffTree<'a> {
    root: &'a dyn HuffBaseNode,
}

impl PartialEq for HuffTree<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.root.eq(other.root)
    }
}

impl Eq for HuffTree<'_> {}

impl PartialOrd for HuffTree<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffTree<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.root.weight() < other.root.weight() {
            Some(Ordering::Less)
        } else if self.root.weight() == other.root.weight() {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}
