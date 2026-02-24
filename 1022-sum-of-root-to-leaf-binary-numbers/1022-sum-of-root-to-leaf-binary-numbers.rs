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
    fn build(
        node: &Rc<RefCell<TreeNode>>,
        mut num: i32,
    ) -> i32 {
        let node = node.borrow();
        
        num <<= 1;
        num |= node.val;

        if node.left.is_none() && node.right.is_none() {
            return num;
        }

        let left_num = if let Some(left_node) = &node.left {
            Self::build(&left_node, num)
        } else {
            0
        };

        let right_num = if let Some(right_node) = &node.right {
            Self::build(&right_node, num)
        } else {
            0
        };

        left_num + right_num
    }
    pub fn sum_root_to_leaf(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> i32 {
        let root = root.unwrap();

        Self::build(&root, 0)
    }
}