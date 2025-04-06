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
    fn find_leaf(cur_list: &mut Vec<i32>, opt_node: Option<Rc<RefCell<TreeNode>>>, master_list: &mut Vec<i32>) {
        if opt_node.is_none() {
            return;
        }

        let node_rc = opt_node.unwrap();
        let node = node_rc.borrow();

        cur_list.insert(0, node.val);

        if node.left.is_none() && node.right.is_none() {
            for i in 0 .. cur_list.len().min(master_list.len()) {
                if cur_list[i] == master_list[i] {
                    continue;
                }

                if cur_list[i] < master_list[i] {
                    *master_list = cur_list.to_vec();
                }
                cur_list.remove(0);

                return;
            }



            if master_list.len() == 0 || (cur_list.len() > 0 && cur_list.len() < master_list.len()) {
                *master_list = cur_list.to_vec();
            }
            cur_list.remove(0);
            return;
        } else {
            Self::find_leaf(cur_list, node.left.clone(), master_list);
            Self::find_leaf(cur_list, node.right.clone(), master_list);
        }

        cur_list.remove(0);
    }
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut master_list = Vec::new();

        Self::find_leaf(&mut Vec::new(), root, &mut master_list);

        master_list.into_iter().map(|v| ('a' as u8 + v as u8) as char).collect()
    }
}