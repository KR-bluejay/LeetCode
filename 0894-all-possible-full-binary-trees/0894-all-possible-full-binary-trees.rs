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
    fn build_tree(node_count: i8) 
    -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = Vec::new();

        if node_count == 1 {
            result.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));

            return result;
        }


        for left_count in 1 .. (node_count - 1) {
            let right_count = node_count - 1 - left_count;
            let left_nodes = Self::build_tree(left_count);
            let right_nodes = Self::build_tree(right_count);
            
            for left_node in left_nodes.iter() {
                for right_node in right_nodes.iter() {
                    result.push(Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: left_node.clone(),
                        right: right_node.clone(),
                    }))));
                }
            }
        }

        result
    }
    pub fn all_possible_fbt(
        node_count: i32
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let node_count = node_count as i8;

        Self::build_tree(node_count)
    }
}