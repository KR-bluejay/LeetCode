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
    fn search_subtree(
        mut root: Option<Rc<RefCell<TreeNode>>>, 
        min_val: i32,
        max_val: i32,
    ) -> (bool, i32, i32) {
        if root.is_none() {
            return (true, 0, 0);
        }
        let root_rc= root.unwrap();
        let root_node = root_rc.borrow();
        
        let (left_valid, left_min_val, left_max_val) = 
            Self::search_subtree(root_node.left.clone(), i32::MIN, root_node.val);
        let (right_valid, right_min_val, right_max_val) = 
            Self::search_subtree(root_node.right.clone(), root_node.val, i32::MAX);

        let is_left_valid = left_valid && left_min_val <= left_max_val &&  left_max_val < root_node.val;
        let is_right_valid = right_valid && root_node.val < right_min_val && right_min_val <= right_max_val;

        if root_node.left.is_none() && root_node.right.is_none() {
            return (true, root_node.val, root_node.val);
        }

        if root_node.left.is_some() && root_node.right.is_some() {
            return (is_left_valid && is_right_valid, left_min_val, right_max_val);
        }

        if root_node.left.is_some() && root_node.right.is_none() {
            return (is_left_valid, left_min_val, root_node.val);
        }


        return (is_right_valid, root_node.val, right_max_val);
    }
    pub fn is_valid_bst(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        Self::search_subtree(root, i32::MAX, i32::MIN).0
    }
}