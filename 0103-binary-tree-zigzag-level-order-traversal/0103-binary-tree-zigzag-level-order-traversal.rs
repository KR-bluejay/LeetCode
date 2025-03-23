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

#[derive(Debug, PartialEq)]
enum TraverseOrder {
    LEFT,
    RIGHT
}

impl Solution {
    pub fn zigzag_level_order(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> Vec<Vec<i32>> {
        let mut direction: TraverseOrder = TraverseOrder::LEFT;
        let mut zigzag_list: Vec<Vec<i32>> = Vec::new();
        let mut node_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut cur_node_list: Vec<i32> = Vec::new();
        
        let mut cur_node_count = 0;
        let mut next_node_count = 0;

        if let Some(root_node) = root {
            cur_node_count += 1;
            node_queue.push_back(root_node.clone());
        }

        while let Some(node_pointer) = node_queue.pop_front() {
            let node = node_pointer.borrow();


            if cur_node_count == 0 {
                cur_node_count = next_node_count;
                next_node_count = 0;

                //zigzag_list.push(cur_node_list.clone());


                direction = if direction == TraverseOrder::LEFT {
                    zigzag_list.push(cur_node_list.clone());
                    TraverseOrder::RIGHT
                } else {
                    let mut a = cur_node_list.clone();
                    a.reverse();
                    zigzag_list.push(a);
                    TraverseOrder::LEFT
                };
                cur_node_list = Vec::new();
            }

            cur_node_count -= 1;
            cur_node_list.push(node.val);


            // if direction == TraverseOrder::RIGHT {
            //     if let Some(node_right) = &node.right {
            //         node_queue.push_back(node_right.clone());
            //         next_node_count += 1;
            //     }

            //     if let Some(node_left) = &node.left {
            //         node_queue.push_back(node_left.clone());
            //         next_node_count += 1;
            //     }
            // } else {
                if let Some(node_left) = &node.left {
                    node_queue.push_back(node_left.clone());
                    next_node_count += 1;
                }

                if let Some(node_right) = &node.right {
                    node_queue.push_back(node_right.clone());
                    next_node_count += 1;
                }
            // }
        }
        if cur_node_list.len() > 0 {
            let mut cloned_list = cur_node_list.clone();

            if direction == TraverseOrder::RIGHT {
                cloned_list.reverse();
            }
            zigzag_list.push(cloned_list);
        }

        zigzag_list
    }
}