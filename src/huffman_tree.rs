use std::cmp::Ordering;
use std::collections::HashMap;

trait Node {
    type Element;

    fn weight(&self) -> usize;
    fn is_leaf(&self) -> bool;
}

#[derive(Debug, Default)]
pub struct HuffmanTree {
    value: Option<usize>,
    element: Option<char>,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl HuffmanTree {
    fn new_leaf(element: char, value: usize) -> Self {
        Self {
            element: Some(element),
            value: Some(value),
            ..Self::default()
        }
    }

    fn new_root(left: Option<Self>, right: Option<Self>) -> Self {
        Self {
            left: left.map(Box::new),
            right: right.map(Box::new),
            element: None,
            value: None,
        }
    }

    fn new_tree(first: Option<Self>, second: Option<Self>) -> Option<Self> {
        match (first, second) {
            (None, None) => None,
            (Some(first_tree), None) => Some(first_tree),
            (None, Some(second_tree)) => Some(second_tree),
            (Some(first_tree), Some(second_tree)) => {
                let tree = match first_tree.weight().cmp(&second_tree.weight()) {
                    Ordering::Less => Self::new_root(Some(first_tree), Some(second_tree)),
                    Ordering::Equal => Self::new_root(Some(first_tree), Some(second_tree)),
                    Ordering::Greater => Self::new_root(Some(second_tree), Some(first_tree)),
                };

                Some(tree)
            }
        }
    }

    pub fn from_frequencies(mut frequencies: Vec<(char, usize)>) -> Self {
        // ordered lowest to highest by value
        frequencies.sort_by(|(_ak, av), (_bk, bv)| av.cmp(bv));

        let _leaves: Vec<HuffmanTree> = frequencies
            .iter()
            .map(|(element, value)| HuffmanTree::new_leaf(*element, *value))
            .collect();

        while _leaves.len() > 1 {
            let subtree_1 = _leaves
                .first()
                .expect("Grabbing first element of list of length > 1");
            let subtree_2 = _leaves
                .get(1)
                .expect("Grabbing second element of list of length > 1");

            let new_tree = Self::new_tree(Some(subtree_1), Some(subtree_2));
        }

        // need to iterate over loop, combining first 2 leaves into a tree, insert it back into
        // correct position for order, then continue iteration on next 2 nodes until we have our
        // tree

        todo!()
    }

    pub fn prefix_codes(&self, parent_prefix: Option<&str>) -> HashMap<char, String> {
        let mut map = HashMap::new();

        match self.is_leaf() {
            true => {
                map.insert(
                    self.element.expect("Accessing element on leaf node"),
                    parent_prefix.unwrap_or("").to_string(),
                );
                map
            }
            false => match (self.left.as_ref(), self.right.as_ref()) {
                (None, None) => map,
                (Some(left_subtree), None) => {
                    let left_subtree_prefixes = &(parent_prefix.unwrap_or("").to_owned() + "0");
                    map.extend(left_subtree.prefix_codes(Some(left_subtree_prefixes)));
                    map
                }
                (None, Some(right_subtree)) => {
                    map.extend(
                        right_subtree
                            .prefix_codes(Some(&(parent_prefix.unwrap_or("").to_owned() + "1"))),
                    );
                    map
                }
                (Some(left_subtree), Some(right_subtree)) => {
                    map.extend(
                        left_subtree
                            .prefix_codes(Some(&(parent_prefix.unwrap_or("").to_owned() + "0"))),
                    );
                    map.extend(
                        right_subtree
                            .prefix_codes(Some(&(parent_prefix.unwrap_or("").to_owned() + "1"))),
                    );

                    map
                }
            },
        }
    }
}

impl Node for HuffmanTree {
    type Element = char;

    fn weight(&self) -> usize {
        if self.value.is_some() {
            self.value.unwrap()
        } else {
            match (self.left.as_ref(), self.right.as_ref()) {
                (None, None) => 0,
                (Some(left_subtree), None) => left_subtree.weight(),
                (None, Some(right_subtree)) => right_subtree.weight(),
                (Some(left_subtree), Some(right_subtree)) => {
                    left_subtree.weight() + right_subtree.weight()
                }
            }
        }
    }

    fn is_leaf(&self) -> bool {
        self.element.is_some()
    }
}

impl PartialEq for HuffmanTree {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
            && self.left == other.left
            && self.right == other.right
            && self.element == other.element
    }
}

impl Eq for HuffmanTree {}

impl PartialOrd for HuffmanTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let weight_comparison = self.weight().cmp(&other.weight());
        match weight_comparison {
            Ordering::Equal => self.element.cmp(&other.element),
            _ => weight_comparison,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // value()
    #[test]
    fn test_value_with_single_node_tree() {
        let node_1 = HuffmanTree::new_leaf('a', 5);
        let tree = HuffmanTree::new_root(Some(node_1), None);

        assert_eq!(tree.weight(), 5);
    }

    #[test]
    fn test_value_for_multi_node_tree() {
        let leaf_1 = HuffmanTree::new_leaf('b', 20);
        let leaf_2 = HuffmanTree::new_leaf('g', 32);
        let tree = HuffmanTree::new_root(Some(leaf_1), Some(leaf_2));

        assert_eq!(tree.weight(), 52);
    }

    // new_tree()
    #[test]
    fn test_new_tree_with_single_leaf() {
        let node_1 = HuffmanTree::new_leaf('a', 5);
        let tree = HuffmanTree::new_tree(Some(node_1), None).expect("Single node tree");

        assert_eq!(tree, HuffmanTree::new_leaf('a', 5));
        assert_eq!(tree.weight(), 5);
    }

    #[test]
    fn test_new_tree_for_two_leaf_nodes() {
        let node_1 = HuffmanTree::new_leaf('a', 5);
        let node_2 = HuffmanTree::new_leaf('z', 26);
        let tree = HuffmanTree::new_tree(Some(node_1), Some(node_2)).expect("Two leaf tree");

        assert_eq!(
            tree,
            HuffmanTree::new_root(
                Some(HuffmanTree::new_leaf('a', 5)),
                Some(HuffmanTree::new_leaf('z', 26))
            )
        );
        assert_eq!(tree.weight(), 31);
    }

    // from_frequencies
    #[test]
    fn test_two_level_huffman_tree_from_frequencies() {
        let frequencies = vec![('d', 42), ('e', 120), ('l', 42), ('u', 37)];

        let e_node = HuffmanTree::new_leaf('e', 120);
        let d_node = HuffmanTree::new_leaf('d', 42);
        let l_node = HuffmanTree::new_leaf('l', 42);
        let u_node = HuffmanTree::new_leaf('u', 37);

        let u_d_root = HuffmanTree::new_root(Some(u_node), Some(d_node));
        let u_d_l_root = HuffmanTree::new_root(Some(u_d_root), Some(l_node));
        let expected_tree = HuffmanTree::new_root(Some(e_node), Some(u_d_l_root));

        let tree = HuffmanTree::from_frequencies(frequencies);

        assert_eq!(expected_tree, tree);
    }

    #[test]
    fn test_huffman_tree_from_frequencies() {
        let frequencies = vec![
            ('c', 32),
            ('d', 42),
            ('e', 120),
            ('k', 7),
            ('l', 42),
            ('m', 24),
            ('u', 37),
            ('z', 2),
        ];

        let e_node = HuffmanTree::new_leaf('e', 120);
        let d_node = HuffmanTree::new_leaf('d', 42);
        let l_node = HuffmanTree::new_leaf('l', 42);
        let u_node = HuffmanTree::new_leaf('u', 37);
        let c_node = HuffmanTree::new_leaf('c', 32);
        let m_node = HuffmanTree::new_leaf('m', 24);
        let k_node = HuffmanTree::new_leaf('k', 7);
        let z_node = HuffmanTree::new_leaf('z', 2);

        let k_z_root = HuffmanTree::new_root(Some(z_node), Some(k_node));
        let m_k_z_root = HuffmanTree::new_root(Some(k_z_root), Some(m_node));
        let c_m_k_z_root = HuffmanTree::new_root(Some(c_node), Some(m_k_z_root));
        let l_c_m_k_z_root = HuffmanTree::new_root(Some(l_node), Some(c_m_k_z_root));
        let u_d_root = HuffmanTree::new_root(Some(u_node), Some(d_node));
        let u_d_l_c_m_z_k_root = HuffmanTree::new_root(Some(u_d_root), Some(l_c_m_k_z_root));
        let expected_tree = HuffmanTree::new_root(Some(e_node), Some(u_d_l_c_m_z_k_root));

        let tree = HuffmanTree::from_frequencies(frequencies);

        assert_eq!(expected_tree, tree);
    }

    // prefix_codes
    #[test]
    fn test_huffman_tree_prefix_codes_for_simple_tree() {
        let frequencies = vec![('a', 23), ('b', 5)];
        let tree = HuffmanTree::from_frequencies(frequencies);

        let expected_prefix_codes: HashMap<char, String> =
            HashMap::from([('a', "1".to_string()), ('b', "0".to_string())]);
        assert_eq!(tree.prefix_codes(None), expected_prefix_codes);
    }

    #[test]
    fn test_huffman_tree_prefix_codes_multi_level_tree() {
        let frequencies = vec![('c', 32), ('k', 7), ('m', 24), ('z', 2)];
        let tree = HuffmanTree::from_frequencies(frequencies);

        let expected_prefix_codes: HashMap<char, String> = HashMap::from([
            ('c', "0".to_string()),
            ('m', "11".to_string()),
            ('z', "100".to_string()),
            ('k', "101".to_string()),
        ]);

        assert_eq!(tree.prefix_codes(None), expected_prefix_codes);
    }

    #[test]
    fn test_huffman_tree_prefix_codes_multi_branch_tree() {
        let frequencies = vec![('c', 32), ('d', 42), ('l', 42), ('m', 24), ('u', 37)];
        let tree = HuffmanTree::from_frequencies(frequencies);

        let expected_prefix_codes: HashMap<char, String> = HashMap::from([
            ('u', "110".to_string()),
            ('d', "10".to_string()),
            ('l', "0".to_string()),
            ('c', "1111".to_string()),
            ('m', "1110".to_string()),
        ]);

        assert_eq!(tree.prefix_codes(None), expected_prefix_codes);
    }

    #[test]
    fn test_huffman_tree_prefix_codes() {
        let frequencies = vec![
            ('c', 32),
            ('d', 42),
            ('e', 120),
            ('k', 7),
            ('l', 42),
            ('m', 24),
            ('u', 37),
            ('z', 2),
        ];
        let tree = HuffmanTree::from_frequencies(frequencies);

        let expected_prefix_codes: HashMap<char, String> = HashMap::from([
            ('e', "0".to_string()),
            ('u', "100".to_string()),
            ('d', "101".to_string()),
            ('l', "110".to_string()),
            ('c', "1110".to_string()),
            ('m', "11111".to_string()),
            ('z', "111100".to_string()),
            ('k', "111101".to_string()),
        ]);

        println!("{:?}", tree);

        /*
         * HuffmanTree {
         *   value: None,
         *   element: None,
         *   left: Some(HuffmanTree {
         *     value: Some(120), element: Some('e'), left: None, right: None }),
         *   right: Some(HuffmanTree {
         *     value: None, element: None,
         *     left: Some(HuffmanTree {
         *       value: Some(42), element: Some('l'), left: None, right: None
         *     }),
         *     right: Some(HuffmanTree {
         *       value: None,
         *       element: None,
         *       left: Some(HuffmanTree {
         *         value: Some(42), element: Some('d'), left: None, right: None
         *       }),
         *       right: Some(HuffmanTree {
         *         value: None,
         *         element: None,
         *         left: Some(HuffmanTree {
         *           value: Some(37), element: Some('u'), left: None, right: None
         *         }),
         *         right: Some(HuffmanTree {
         *           value: None,
         *           element: None,
         *           left: Some(HuffmanTree {
         *             value: Some(32), element: Some('c'), left: None, right: None
         *           }),
         *           right: Some(HuffmanTree {
         *             value: None,
         *             element: None,
         *             left: Some(HuffmanTree {
         *               value: None,
         *               element: None,
         *               left: Some(HuffmanTree {
         *                 value: Some(2), element: Some('z'), left: None, right: None
         *               }),
         *               right: Some(HuffmanTree {
         *                 value: Some(7), element: Some('k'), left: None, right: None
         *               })
         *             }),
         *             right: Some(HuffmanTree {
         *               value: Some(24), element: Some('m'), left: None, right: None
         *             })
         *           })
         *         })
         *       })
         *     })
         *   })
         * }
         */

        assert_eq!(tree.prefix_codes(None), expected_prefix_codes);
    }
}
