use std::collections::{HashSet};
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn modified_list(
        mut nums: Vec<i32>, 
        mut head: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let num_set: HashSet<i32> = nums.into_iter().collect();

        let mut cur_node = &mut head;

        loop {
            match cur_node {
                Some(node) if num_set.contains(&node.val) => {
                    let next_node = node.next.take();
                    
                    *cur_node = next_node;
                },
                Some(node) => {
                    cur_node = &mut node.next;
                }
                None => break,
            }
        }
        head
    }
}