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
    fn build_tree(
        preorder: &[i32],
    ) -> Rc<RefCell<TreeNode>> {
        let left_id = 1;
        let right_id = preorder.iter()
            .position(|&p| preorder[0] < p)
            .unwrap_or(preorder.len());
        
        Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: if left_id < preorder.len() && left_id < right_id {
                Some(Self::build_tree(&preorder[1 .. right_id]))
            } else {
                None
            },
            right: if right_id < preorder.len() {
                Some(Self::build_tree(&preorder[right_id .. ]))
            } else {
                None
            },
        }))
    }
    pub fn bst_from_preorder(
        preorder: Vec<i32>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Self::build_tree(&preorder))
    }
}