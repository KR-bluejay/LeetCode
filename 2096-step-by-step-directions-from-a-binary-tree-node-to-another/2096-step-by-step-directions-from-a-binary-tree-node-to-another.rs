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
    fn find(
        node: &Rc<RefCell<TreeNode>>,
        value: i32,
        path: &mut Vec<u8>,
    ) -> bool {
        let node = node.borrow();

        if node.val == value {
            return true;
        }

        if let Some(left_node) = &node.left 
        && Self::find(&left_node, value, path) {
            path.push(b'L');
        } else if let Some(right_node) = &node.right 
        && Self::find(&right_node, value, path) {
            path.push(b'R');
        }

        !path.is_empty()
    }
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>, 
        start_value: i32, 
        dest_value: i32
    ) -> String {
        let root = root.unwrap();
        
        let mut src_path = Vec::new();
        let mut dst_path = Vec::new();


        Self::find(&root, start_value, &mut src_path);
        Self::find(&root, dest_value, &mut dst_path);

        while let Some(&src_byte) = src_path.last() 
        && let Some(&dst_byte) = dst_path.last() 
        && src_byte == dst_byte {
            src_path.pop();
            dst_path.pop();
        }

        let mut result = Vec::with_capacity(src_path.len() + dst_path.len());

        for _ in 0 .. src_path.len() {
            result.push(b'U');
        }

        result.extend(dst_path.into_iter().rev());

        unsafe {String::from_utf8_unchecked(result)}
    }
}