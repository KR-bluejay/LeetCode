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
use std::collections::HashMap;

impl Solution {
    pub fn create_binary_tree(
        descriptions: Vec<Vec<i32>>
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node_map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();

        for description in descriptions.into_iter() {
            let parent_id = description[0];
            let mut parent = if let Some(node) = node_map.remove(&parent_id) {
                node
            } else {
                Rc::new(RefCell::new(TreeNode::new(parent_id)))
            };
            let child_id = description[1];
            let child = if let Some(child) = node_map.get(&child_id) {
                child.clone()
            } else {
                let node = Rc::new(RefCell::new(TreeNode::new(child_id)));
                node_map.insert(child_id, node.clone());
                node
            };
            let is_left = description[2] == 1;

            if is_left {
                parent.borrow_mut().left = Some(child);
            } else {
                parent.borrow_mut().right = Some(child);
            }

            node_map.insert(parent_id, parent);
        }

        for node in node_map.into_values() {
            if Rc::strong_count(&node) == 1 {
                return Some(node);
            }
        }

        None
    }
}