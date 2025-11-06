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
use itertools::Itertools;

impl Solution {
    pub fn reverse_k_group(
        head: Option<Box<ListNode>>, 
        k: i32
    ) -> Option<Box<ListNode>> {
        let k = k as usize;

        let mut nums: Vec<i32> = Vec::with_capacity(5000);
        let mut opt_node = head;

        while let Some(mut node) = opt_node {
            nums.push(node.val);
            opt_node = node.next.take();
        }

        let mut dummy_node = Some(Box::new(ListNode {
            val: 0,
            next: None,
        }));
        let mut prev_node = &mut dummy_node;


        for num_chunk in nums.chunks(k) {
            if num_chunk.len() < k {
                for &num in num_chunk.iter() {
                    prev_node.as_mut().unwrap().next = Some(Box::new(ListNode {
                        val: num,
                        next: None,
                    }));
                    prev_node = &mut prev_node.as_mut().unwrap().next;
                }
            } else {
                for &num in num_chunk.iter().rev() {
                    prev_node.as_mut().unwrap().next = Some(Box::new(ListNode {
                        val: num,
                        next: None,
                    }));
                    prev_node = &mut prev_node.as_mut().unwrap().next;
                }
            }
        }


        dummy_node.unwrap().next
    }
}