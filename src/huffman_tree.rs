use std::cmp::Ordering;

trait Node<'a> {
    type Value;

    fn left(&self) -> Option<&Self>;
    fn right(&self) -> Option<&Self>;
    fn value(&self) -> &Self::Value;
}

trait BinaryTree {
    type Node<'a>: Node<'a>
    where
        Self: 'a;

    fn root(&self) -> Option<&Self::Node<'_>>;
}

struct HuffmanTree<'a> {
    root: Option<<HuffmanTree<'a> as BinaryTree>::Node<'a>>,
}

impl BinaryTree for HuffmanTree<'_> {
    type Node<'a> = HuffmanTreeNode<'a> where Self: 'a;

    fn root(&self) -> Option<&Self::Node<'_>> {
        self.root.as_ref()
    }
}

impl PartialEq for HuffmanTree<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.root.clone().into_iter().eq(other.root.clone())
    }
}

impl Eq for HuffmanTree<'_> {}

impl PartialOrd for HuffmanTree<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanTree<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.root.clone().into_iter().cmp(other.root.clone())
    }
}

#[derive(Clone)]
struct HuffmanTreeNode<'a> {
    left: Option<&'a Self>,
    right: Option<&'a Self>,
    value: usize,
}

impl Node<'_> for HuffmanTreeNode<'_> {
    type Value = usize;

    fn left(&self) -> Option<&Self> {
        self.left
    }

    fn right(&self) -> Option<&Self> {
        self.right
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl PartialEq for HuffmanTreeNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for HuffmanTreeNode<'_> {}

impl PartialOrd for HuffmanTreeNode<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanTreeNode<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value < other.value {
            Ordering::Greater
        } else if self.value == other.value {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
