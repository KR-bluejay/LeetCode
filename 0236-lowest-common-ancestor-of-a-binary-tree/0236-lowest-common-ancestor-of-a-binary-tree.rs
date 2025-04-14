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
        root: &Option<Rc<RefCell<TreeNode>>>, 
        p_val: i32, 
        q_val: i32, 
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let root_rc = root.as_ref().unwrap();
        let root_node = root_rc.borrow();

        if root_node.val == p_val || root_node.val == q_val {
            return Some(root_rc.clone());
        }
        
        let left_node = Self::find_node(&root_node.left, p_val, q_val);
        let right_node = Self::find_node(&root_node.right, p_val, q_val);

        if left_node.is_some() && right_node.is_some() {
            return Some(root_rc.clone());
        }

        left_node.or(right_node)
    }
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>, 
        p: Option<Rc<RefCell<TreeNode>>>, 
        q: Option<Rc<RefCell<TreeNode>>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;


        Self::find_node(&root, p_val, q_val)
    }
}