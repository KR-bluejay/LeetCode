use std::collections::HashSet;

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

        while matches!(
            head.as_ref(), 
            Some(node) if num_set.contains(&node.val)
        ) {
            head = head.take()?.next;
        }

        let mut cur_node = head.as_mut();

        while let Some(node) = cur_node {
            while matches!(
                node.next.as_ref(), 
                Some(next_node) if num_set.contains(&next_node.val)
            ) {
                node.next = node.next.take()?.next;
            }
            cur_node = node.next.as_mut();
        }

        head
    }
}