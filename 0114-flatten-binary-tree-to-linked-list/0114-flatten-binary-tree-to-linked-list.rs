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
    fn parse_node(opt_parent_node: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if opt_parent_node.is_none() {
            return None;
        }

        let parent_node_rc = opt_parent_node.as_ref().unwrap();
        let mut parent_node = parent_node_rc.borrow_mut();

        let mut left_node = Self::parse_node(&mut parent_node.left);
        let mut right_node = Self::parse_node(&mut parent_node.right);

        // 자식 노드가 없는 경우, 본인만 반환
        if left_node.is_none() && right_node.is_none() {
            return opt_parent_node.clone();
        }

        if left_node.is_some() && right_node.is_some() {
            let parent_right = parent_node.right.take();

            left_node.as_mut()?.borrow_mut().right = parent_right;

            parent_node.right = parent_node.left.take();

            return right_node.clone();
        }

        if left_node.is_some() && right_node.is_none() {
            let parent_left = parent_node.left.take();
            parent_node.right = parent_left;
            return left_node.clone();
        }

        if left_node.is_none() && right_node.is_some() {
            return right_node.clone();
        }

        None
    }
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::parse_node(root);
    }
}