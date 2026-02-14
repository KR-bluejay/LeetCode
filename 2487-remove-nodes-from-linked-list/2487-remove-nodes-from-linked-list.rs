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
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node_stack: Vec<i32> = Vec::with_capacity(100000);

        while let Some(node) = head {
            while let Some(&last_val) = node_stack.last() 
            && last_val < node.val {
                node_stack.pop();
            }
            node_stack.push(node.val);

            head = node.next;
        }

        let mut last_node = None;

        while let Some(node_val) = node_stack.pop() {
            let mut node = Box::new(ListNode::new(node_val));
            
            node.next = last_node;
            last_node = Some(node);
        }
        

        last_node
    }
}