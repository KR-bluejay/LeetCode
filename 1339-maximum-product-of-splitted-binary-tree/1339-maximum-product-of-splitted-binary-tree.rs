// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn update_tree_val(node: &Rc<RefCell<TreeNode>>) -> i32 {
        let mut node = node.borrow_mut();
        let mut node_val = 0;

        if let Some(ref left_node) = node.left {
            node_val += Self::update_tree_val(left_node);
        }

        if let Some(ref right_node) = node.right {
            node_val += Self::update_tree_val(right_node);
        }

        node.val += node_val;
        node.val
    }
    fn find_max(node: &Rc<RefCell<TreeNode>>, total: i64, max_prod: &mut i64) {
        let node = node.borrow_mut();
        let node_val = node.val as i64;

        (*max_prod) = (*max_prod).max(node_val * (total - node_val));

        if let Some(ref left_node) = node.left {
            Self::find_max(left_node, total, max_prod);
        }

        if let Some(ref right_node) = node.right {
            Self::find_max(right_node, total, max_prod);
        }
    }

    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.unwrap();
        
        let total = Self::update_tree_val(&root) as i64;
        
        let mut max_prod: i64 = 0;

        Self::find_max(&root, total, &mut max_prod);

        (max_prod % 1_000_000_007) as i32
    }
}