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
    #[inline]
    fn connect_tail(
        mut node: Option<Box<ListNode>>,
        rest: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(ref mut n) = node {
            if n.next.is_some() {
                n.next = Self::connect_tail(n.next.take(), rest);
            } else {
                n.next = rest;
            }
        }
        node
    }
    pub fn reverse_k_group(
        head: Option<Box<ListNode>>,
        k: i32,
    ) -> Option<Box<ListNode>> {
        let k = k as usize;
        
        let mut check = &head;
        for _ in 0..k {
            match check {
                Some(node) => check = &node.next,
                None => return head,
            }
        }
        
        let mut prev = None;
        let mut curr = head;
        
        for _ in 0..k {
            curr = match curr {
                Some(mut node) => {
                    let next = node.next.take();
                    node.next = prev;
                    prev = Some(node);
                    next
                }
                None => break,
            };
        }
        
        
        Self::connect_tail(prev, Self::reverse_k_group(curr, k as i32))
    }
}