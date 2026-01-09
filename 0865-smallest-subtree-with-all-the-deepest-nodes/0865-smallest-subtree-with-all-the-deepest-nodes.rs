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
    fn find_deepest_sub_tree(
        node_rc: &Rc<RefCell<TreeNode>>,
    ) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        let node = node_rc.borrow();

        let left_sub = if let Some(node_left) = &node.left {
            Self::find_deepest_sub_tree(
                &node_left
            )
        } else {
            (0, None)
        };
        let right_sub = if let Some(node_right) = &node.right {
            Self::find_deepest_sub_tree(
                &node_right
            )
        } else {
            (0, None)
        };

        match left_sub.0.cmp(&right_sub.0) {
            Ordering::Less => (right_sub.0 + 1, right_sub.1),
            Ordering::Equal => (left_sub.0 + 1, Some(node_rc.clone())),
            Ordering::Greater => (left_sub.0 + 1, left_sub.1),
        }
    }
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_deepest_sub_tree(&root.unwrap()).1
    }
}