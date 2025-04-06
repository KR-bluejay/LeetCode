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
    fn is_better_path(path: &Vec<i32>, best_path: &Vec<i32>) -> bool {
        let reversed_path = path.iter().rev();
        let reversed_best_path = best_path.iter().rev();

        best_path.len() == 0 || 
        reversed_path.zip(reversed_best_path)
            .find(|(&a, &b)| a != b)
            .map_or_else(
                || path.len() < best_path.len(), 
                |(a, b)| a < b
            )
    }
    fn parse_smallest_string(
        opt_node_ref: Option<Rc<RefCell<TreeNode>>>, 
        path: &mut Vec<i32>, 
        best_path: &mut Vec<i32>
    ) {
        let node_ref = match opt_node_ref {
            Some(n) => n,
            None => return,
        };
        let node = node_ref.borrow();

        path.push(node.val);

        if node.left.is_none() && node.right.is_none() && Self::is_better_path(path, best_path) {
            best_path.clear();
            best_path.extend_from_slice(&path);
        }

        Self::parse_smallest_string(node.left.clone(), path, best_path);
        Self::parse_smallest_string(node.right.clone(), path, best_path);

        path.pop();
    }
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut path = Vec::with_capacity(8500);
        let mut best_path = Vec::with_capacity(8500);

        Self::parse_smallest_string(root, &mut path, &mut best_path);

        best_path.into_iter().rev()
            .map(|v| ('a' as u8 + v as u8) as char)
            .collect()
    }
}