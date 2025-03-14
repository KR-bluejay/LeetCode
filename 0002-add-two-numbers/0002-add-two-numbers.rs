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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>, 
        mut l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut head_node = ListNode::new(0);
        let mut current_node = &mut head_node;
        let mut carry_num = 0;

        while l1.is_some() || l2.is_some() || carry_num > 0 {
            let ((first_val, first_node_next), (second_val, second_node_next)) = (
                l1.map(|node| (node.val, node.next)).unwrap_or((0, None)),
                l2.map(|node| (node.val, node.next)).unwrap_or((0, None)),
            );

            let total = first_val + second_val + carry_num;
            let current_val = total % 10;
            carry_num = total / 10;


            current_node.next = Some(Box::new(ListNode::new(current_val)));

            if let Some(ref mut next_node) = current_node.next {
                current_node = next_node;
            }

            l1 = first_node_next;
            l2 = second_node_next;
        }

        return head_node.next;
    }
}