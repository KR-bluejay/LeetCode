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
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        let mut nodes = Vec::new();

        while let Some(node) = head {
            nodes.push(node.val);
            head = node.next;
        }

        for id in 0 .. nodes.len() / 2 {
            let left = nodes[id];
            let right = nodes[nodes.len() - id - 1];

            result = result.max(left + right);
        }

        result
    }
}