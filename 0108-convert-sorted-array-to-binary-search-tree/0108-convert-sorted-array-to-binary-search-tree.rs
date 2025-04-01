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
    fn build_node(left: usize, right: usize, nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match left.cmp(&right) {
            Ordering::Greater => None,
            Ordering::Equal => Some(Rc::new(RefCell::new(TreeNode::new(nums[left])))),
            Ordering::Less => {
                let mid = (left + right) / 2;
                let mut new_node = TreeNode::new(nums[mid]);
        
                new_node.left = if left < mid {
                    Self::build_node(left, mid - 1, &nums)
                } else {
                    None
                };

                new_node.right = if mid < right {
                    Self::build_node(mid + 1, right, &nums)
                } else {
                    None
                };
        
                Some(Rc::new(RefCell::new(new_node)))
            },
        }
    }
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let left = 0;
        let right = nums.len() - 1;

        Self::build_node(left, right, &nums)
    }
}