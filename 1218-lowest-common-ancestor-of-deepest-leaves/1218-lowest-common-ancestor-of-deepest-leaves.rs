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
use std::cmp::Ordering;

impl Solution {
    fn find_lca(opt_target_node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        let target_node_rc: Rc<RefCell<TreeNode>> = match opt_target_node {
            Some(node) => node,
            None => return (0, None)
        };
        let target_node = target_node_rc.borrow();

        if target_node.left.is_none() && target_node.right.is_none() {
            return (
                1, 
                Some(target_node_rc.clone())
            );
        }

        let (left_depth, left_lca_node) = Self::find_lca(target_node.left.clone());
        let (right_depth, right_lca_node) = Self::find_lca(target_node.right.clone());

        match left_depth.cmp(&right_depth) {
            Ordering::Less => return (1 + right_depth, right_lca_node),
            Ordering::Equal => return (1 + left_depth, Some(target_node_rc.clone())),
            Ordering::Greater => return (1 + left_depth, left_lca_node),
        };

        (0, None)
    }
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (lca_depth, lca_node) = Self::find_lca(root);

        println!("{lca_depth}, {lca_node:?}");

        lca_node
    }
}