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
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut node_id = 0;
        let mut node_stack: Vec<(usize, i32)> = Vec::with_capacity(100000);
        let mut results: Vec<i32> = Vec::with_capacity(100000);

        while let Some(mut node) = head {
            while let Some(&(last_node_id, last_node_val)) = node_stack.last() 
            && last_node_val < node.val {
                println!("{node_id} {last_node_id}");
                results[last_node_id] = node.val;

                node_stack.pop();
            }

            

            results.push(0);
            node_stack.push((node_id, node.val));
            
            node_id += 1;
            head = node.next.take();
        }

        results
    }
}