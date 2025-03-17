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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast_node = head.as_ref();
        let mut slow_node = head.as_ref();

        while let Some(next_node) = fast_node {
            fast_node = next_node.next.as_ref();
            
            if let Some(temp_node) = fast_node {
                fast_node = temp_node.next.as_ref();
                slow_node = slow_node.unwrap().next.as_ref();
            }
        }

        return slow_node.clone().cloned();
    }
}