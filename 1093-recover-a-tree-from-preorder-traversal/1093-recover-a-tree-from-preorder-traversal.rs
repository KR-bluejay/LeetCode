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
    fn build_tree(nodes: &[(usize, i32)]) -> Option<Rc<RefCell<TreeNode>>> {
        if nodes.len() == 0 {
            return None;
        }

        let (root_depth, root_val) = nodes[0];
        let child_node_ids: Vec<usize> = nodes.iter()
            .enumerate()
            .filter_map(|(id, (d, v))| {
                return if *d == root_depth + 1 {
                    Some(id)
                } else {
                    None 
                }
            })
            .collect();

        let (left_node, right_node) = if child_node_ids.len() == 0 { 
            (None, None)
        } else if child_node_ids.len() == 1 {
            (Self::build_tree(&nodes[child_node_ids[0] .. ]), None)
        } else {
            (
                Self::build_tree(&nodes[child_node_ids[0] .. child_node_ids[1]]), 
                Self::build_tree(&nodes[child_node_ids[1]..])
            )
        };


        Some(Rc::new(RefCell::new(TreeNode {val: root_val, left: left_node, right: right_node})))
    }
    pub fn recover_from_preorder(
        traversal: String
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let traversal: Vec<char> = traversal.chars().collect();
        let mut nodes: Vec<(usize, i32)> = Vec::new();

        let mut node_depth = 0;

        for mut traversal_id in 0 .. traversal.len() {
            if traversal[traversal_id] == '-' {
                node_depth += 1;
                continue;
            }

            let mut cur_node_val = 0;

            while traversal_id < traversal.len() 
                && traversal[traversal_id] != '-' {
                cur_node_val = cur_node_val * 10 
                    + traversal[traversal_id].to_digit(10).unwrap() as i32;
                traversal_id += 1;
            }


            nodes.push((node_depth, cur_node_val));
            node_depth = 0;
        }

        Self::build_tree(&nodes)
    }
}