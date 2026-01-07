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
    fn modify_tree(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node = node.unwrap();
        let mut node = node.borrow_mut();

        let left_val = Self::modify_tree(node.left.clone());
        let right_val = Self::modify_tree(node.right.clone());

        node.val = (node.val + left_val + right_val);
        node.val
    }
    fn find_max_product(
        node: Option<Rc<RefCell<TreeNode>>>,
        total: i32,
        max_product: &mut i64,
    ) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let node = node.borrow();

        const MODULO: i64 = 1_000_000_007;


        Self::find_max_product(node.left.clone(), total, max_product);
        Self::find_max_product(node.right.clone(), total, max_product);

        if let Some(left_node) = &node.left {
            let left_node = left_node.borrow();
            let left_val = left_node.val;

            (*max_product) = (*max_product)
                .max((total - left_val) as i64 * left_val as i64);
        }

        if let Some(right_node) = &node.right {
            let right_node = right_node.borrow();
            let right_val = right_node.val;

            (*max_product) = (*max_product)
                .max((total - right_val) as i64 * right_val as i64);
        }

    }
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_product: i64 = 0;
        const MODULO: i64 = 1_000_000_007;


        let total = Self::modify_tree(root.clone());
        Self::find_max_product(root, total, &mut max_product);


        (max_product % MODULO) as i32
    }
}