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
    fn find_node(
        root: Option<Rc<RefCell<TreeNode>>>, 
        p_val: i32, 
        q_val: i32, 
        visited: &mut Vec<bool>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let root_rc = root.unwrap();
        let root_node = root_rc.borrow();

        let left_node = Self::find_node(root_node.left.clone(), p_val, q_val, visited);
        let right_node = Self::find_node(root_node.right.clone(), p_val, q_val, visited);

        if left_node.is_some() && right_node.is_some() {
            return Some(root_rc.clone());
        }

        if (root_node.val == p_val || root_node.val == q_val) && (left_node.is_some() || right_node.is_some()) {
            return Some(root_rc.clone());
        }

        if left_node.is_some() {
            return left_node;
        }

        if right_node.is_some() {
            return right_node;
        }

        if root_node.val == p_val || root_node.val == q_val {
            return Some(root_rc.clone());
        }

        return None;
    }
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>, 
        p: Option<Rc<RefCell<TreeNode>>>, 
        q: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        let mut visited: Vec<bool> = vec![false; 2];

        Self::find_node(root, p_val, q_val, &mut visited)
    }
}