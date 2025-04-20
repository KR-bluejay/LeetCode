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
    fn build_tree(traversal: &[&str], depth: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if traversal.len() == 0 {
            return None;
        }

        println!("{traversal:?}");
        let root_val = traversal[0].parse::<i32>().unwrap();
        let mut cur_count = 0;
        let mut left_id = 0;
        let mut right_id = 0;

        for i in 1 .. traversal.len() {
            if traversal[i] == "" {
                cur_count += 1;
                continue;
            }


            if cur_count == depth && left_id == 0 {
                left_id = i;
                cur_count = 0;
                continue;
            }

            if cur_count == depth && right_id == 0 {
                right_id = i;
                cur_count = 0;
                break;
            }

            if traversal[i] != "" {
                cur_count = 0;
                continue;
            }
        }

        if right_id == 0 {
            right_id = traversal.len();
        }

        let mut left_node = if left_id != 0 {
            Self::build_tree(&traversal[left_id .. right_id], depth + 1)
        } else {
            None
        };
        let mut right_node = if left_id != 0 {
            Self::build_tree(&traversal[right_id .. traversal.len()], depth + 1)
        } else {
            None
        };



        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: left_node,
            right: right_node,
        })))

        
    }
    pub fn recover_from_preorder(
        traversal: String
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let v: Vec<&str> = traversal.split('-').collect();
        Self::build_tree(&v, 0)
    }
}