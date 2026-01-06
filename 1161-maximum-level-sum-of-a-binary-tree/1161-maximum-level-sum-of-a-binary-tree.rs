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

use std::collections::VecDeque;

impl Solution {
    pub fn max_level_sum(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> i32 {
        let mut max_level = 1;
        let mut max_sum = i32::MIN;
        
        let mut cur_level = 1;

        let mut cur_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::with_capacity(1000);

        cur_queue.push_back(root.unwrap());

        while !cur_queue.is_empty() {
            let mut cur_sum = 0;

            for id in 0 .. cur_queue.len() {
                let cur_node = cur_queue.pop_front().unwrap();
                let cur_node = cur_node.borrow();

                cur_sum += cur_node.val;
                
                if let Some(left_node) = &cur_node.left {
                    cur_queue.push_back(left_node.clone());
                }
    
                if let Some(right_node) = &cur_node.right {
                    cur_queue.push_back(right_node.clone());
                }
            }

            if cur_sum > max_sum {
                max_sum = cur_sum;
                max_level = cur_level;
            }
            
            cur_level += 1;
        }

        max_level
    }
}