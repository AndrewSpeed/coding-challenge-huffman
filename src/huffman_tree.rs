use std::cmp::Ordering;
use std::collections::HashMap;

#[warn(unused_variables)]
trait Node<'a> {
    type Value;
    type Element;

    fn left(&self) -> Option<&Self>;
    fn right(&self) -> Option<&Self>;
    fn value(&self) -> &Self::Value;
    fn element(&self) -> Option<&Self::Element>;
    fn is_leaf(&self) -> bool;
}

trait BinaryTree {
    type Node<'a>: Node<'a>
    where
        Self: 'a;

    fn root(&self) -> Option<&Self::Node<'_>>;
}

#[derive(Debug)]
pub struct HuffmanTree<'a> {
    root: Option<<HuffmanTree<'a> as BinaryTree>::Node<'a>>,
}

impl HuffmanTree<'_> {
    pub fn from_frequency_map(freq_map: HashMap<char, usize>) -> Self {
        let mut ordered_elements: Vec<(char, usize)> = freq_map.into_iter().collect();
        // ordered lowest to highest by value
        ordered_elements.sort_by(|(_ak, av), (_bk, bv)| av.cmp(bv));

        let root = None;
        Self { root }
    }
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

#[derive(Clone, Debug, Default)]
struct HuffmanTreeNode<'a> {
    left: Option<&'a Self>,
    right: Option<&'a Self>,
    value: usize,
    element: Option<char>,
}

impl<'a> HuffmanTreeNode<'a> {
    pub fn new_leaf_node(element: char, value: usize) -> Self {
        Self {
            element: Some(element),
            value,
            ..Self::default()
        }
    }

    pub fn new_root_node(left: Option<&'a Self>, right: Option<&'a Self>) -> Self {
        let value = left.unwrap_or(&HuffmanTreeNode::default()).value
            + right.unwrap_or(&HuffmanTreeNode::default()).value;
        Self {
            left,
            right,
            value,
            ..Self::default()
        }
    }
}

impl Node<'_> for HuffmanTreeNode<'_> {
    type Value = usize;
    type Element = char;

    fn left(&self) -> Option<&Self> {
        self.left
    }

    fn right(&self) -> Option<&Self> {
        self.right
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn element(&self) -> Option<&Self::Element> {
        self.element.as_ref()
    }

    fn is_leaf(&self) -> bool {
        self.element.is_some()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_tree_from_frequency_map() {
        let map: HashMap<char, usize> = HashMap::from([
            ('c', 32),
            ('d', 42),
            ('e', 120),
            ('k', 7),
            ('l', 42),
            ('m', 24),
            ('u', 37),
            ('z', 2),
        ]);

        let e_node = HuffmanTreeNode::new_leaf_node('e', 120);
        let d_node = HuffmanTreeNode::new_leaf_node('d', 42);
        let l_node = HuffmanTreeNode::new_leaf_node('l', 42);
        let u_node = HuffmanTreeNode::new_leaf_node('u', 37);
        let c_node = HuffmanTreeNode::new_leaf_node('c', 32);
        let m_node = HuffmanTreeNode::new_leaf_node('m', 24);
        let k_node = HuffmanTreeNode::new_leaf_node('k', 7);
        let z_node = HuffmanTreeNode::new_leaf_node('z', 2);

        let k_z_root = HuffmanTreeNode::new_root_node(Some(&z_node), Some(&k_node));
        let m_k_z_root = HuffmanTreeNode::new_root_node(Some(&k_z_root), Some(&m_node));
        let c_m_k_z_root = HuffmanTreeNode::new_root_node(Some(&c_node), Some(&m_k_z_root));
        let l_c_m_k_z_root = HuffmanTreeNode::new_root_node(Some(&l_node), Some(&c_m_k_z_root));
        let u_d_root = HuffmanTreeNode::new_root_node(Some(&u_node), Some(&d_node));
        let u_d_l_c_m_z_k_root =
            HuffmanTreeNode::new_root_node(Some(&u_d_root), Some(&l_c_m_k_z_root));
        let root = HuffmanTreeNode::new_root_node(Some(&e_node), Some(&u_d_l_c_m_z_k_root));
        let expected_tree = HuffmanTree { root: Some(root) };

        let tree = HuffmanTree::from_frequency_map(map);

        assert_eq!(expected_tree, tree);
    }
}
