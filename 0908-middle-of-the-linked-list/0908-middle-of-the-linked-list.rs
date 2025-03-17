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
        let mut tail_node = head.as_ref();
        let mut mid_node = head.as_ref();
        
        let mut total_node_count = 1;
        let mut mid_node_count = 1;

        while tail_node.is_some() {
            tail_node = tail_node.unwrap().next.as_ref();
            total_node_count += 1;

            let temp: i32 = ((total_node_count as f32) / (2 as f32)).round() as i32;

            for i in mid_node_count .. temp {
                println!("{total_node_count} {i}");

                mid_node = mid_node.unwrap().next.as_ref();
            }

            mid_node_count = temp;
        }

        return mid_node.clone().cloned();
    }
}