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

use std::collections::{ VecDeque };

impl Solution {
    pub fn zigzag_level_order(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> Vec<Vec<i32>> {
        let mut node_level: usize = 0;

        let mut node_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut zigzag_list: Vec<Vec<i32>> = Vec::new();

        if let Some(root_node) = root {
            node_queue.push_front(root_node.clone());
        }

        while !node_queue.is_empty() {
            let cur_node_count = node_queue.len();
            let mut cur_node_list: Vec<i32> = Vec::with_capacity(cur_node_count);

            for i in 0 .. cur_node_count {
                if let Some(node_ref) = node_queue.pop_front() {
                    let node = node_ref.borrow();

                    if node_level % 2 == 0 {
                        cur_node_list.push(node.val);
                    } else {
                        cur_node_list.insert(0, node.val);
                    }

                    node.left.as_ref().map(|left_node_ref| node_queue.push_back(left_node_ref.clone()));
                    node.right.as_ref().map(|right_node_ref| node_queue.push_back(right_node_ref.clone()));
                }
            }

            node_level += 1;
            zigzag_list.push(cur_node_list);
        }

        zigzag_list
    }
}