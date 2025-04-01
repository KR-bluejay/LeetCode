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
    fn build_node(left: usize, right: usize, nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mid = (left + right) / 2;

        if left == right {
            let new_node = TreeNode::new(nums[left]);

            return Some(Rc::new(RefCell::new(new_node)));
        }

        if left + 1 == right {
            let mut new_node = TreeNode::new(nums[right]);

            new_node.left = Self::build_node(left, left, nums);

            return Some(Rc::new(RefCell::new(new_node)));
        }

        let mut new_node = TreeNode::new(nums[mid]);

        new_node.left = Self::build_node(left, mid - 1, &nums);
        new_node.right = Self::build_node(mid + 1, right, &nums);

        return Some(Rc::new(RefCell::new(new_node)));
    }
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut left = 0;
        let mut right = nums.len() - 1;

        Self::build_node(left, right, &nums)
    }
}