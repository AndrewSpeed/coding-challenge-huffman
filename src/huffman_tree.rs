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
