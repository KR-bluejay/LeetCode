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
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let n = n as usize;

        let mut tree_nodes: Vec<Vec<Option<Rc<RefCell<TreeNode>>>>> 
            = vec![vec![]; n + 1];

        tree_nodes[1].push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
        
        for total_id in (3 ..= n).step_by(2) {
            let mut tree = Vec::new();

            for left_id in (1 .. (total_id - 1)).step_by(2) {
                let right_id = total_id - left_id - 1;

                tree.reserve(tree_nodes[left_id].len() * tree_nodes[right_id].len());

                for left_node in tree_nodes[left_id].iter() {
                    for right_node in tree_nodes[right_id].iter() {
                        tree.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: left_node.clone(),
                            right: right_node.clone()
                        }))));
                    }
                }
            }

            tree_nodes[total_id] = tree
        }

        tree_nodes.pop().unwrap()
    }
}