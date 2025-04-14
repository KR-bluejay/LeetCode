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
    fn rebuild_subnode(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 || postorder.len() == 0 {
            return None;
        }

        let post_root_val = postorder[postorder.len() - 1];
        let in_root_id = inorder.iter().position(|&v| v == post_root_val).unwrap_or(0);

        let (in_left_node, in_right_node) = (&inorder[0 .. in_root_id], &inorder[in_root_id + 1 .. ]);
        let (post_left_node, post_right_node) = (&postorder[0 .. in_left_node.len()], &postorder[in_left_node.len() .. postorder.len() - 1]);


        Some(
            Rc::new(
                RefCell::new(
                    TreeNode {
                        val: post_root_val,
                        left: Self::rebuild_subnode(in_left_node, post_left_node),
                        right: Self::rebuild_subnode(in_right_node, post_right_node),
                    }
                )
            )
        )
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::rebuild_subnode(&inorder, &postorder)
    }
}