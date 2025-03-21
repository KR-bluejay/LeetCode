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
use std::cell::{Ref, RefCell};

use std::collections::{VecDeque};

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut cur_node_count = 0;
        let mut cur_node_list: Vec<i32> = Vec::new();
        let mut total_node_list: Vec<Vec<i32>> = Vec::new();

        let mut next_node_count = 0;

        
        let mut node_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(root_node) = root {
            node_queue.push_back(root_node);
            cur_node_count += 1;
        }

        while !node_queue.is_empty() {
            let target_node = node_queue.pop_front().unwrap();
            let borrow_node = target_node.borrow();


            if cur_node_count == 0  {
                cur_node_count = next_node_count;
                next_node_count = 0;
                
                total_node_list.push(cur_node_list.clone());
                cur_node_list.clear();
            }
            cur_node_count -= 1;
            cur_node_list.push(target_node.borrow().val);

            if let Some(left_node) = &borrow_node.left {
                node_queue.push_back(Rc::clone(left_node));
                next_node_count += 1;
            }

            if let Some(right_node) = &borrow_node.right {
                node_queue.push_back(Rc::clone(right_node));
                next_node_count += 1;
            }
        }

        if cur_node_list.len() > 0 {
            total_node_list.push(cur_node_list.clone());
        }
        
        total_node_list
    }
}