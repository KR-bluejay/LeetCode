use std::collections::BinaryHeap;
use std::cmp::Ordering;


struct Node {
    val: i64,
    prev_id: Option<usize>,
    next_id: Option<usize>,
    is_merged: bool,
}

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    left_id: usize,
    right_id: usize,
    cost: i64,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
            .then(self.left_id.cmp(&other.left_id).reverse())
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let num_last_id = nums.len() - 1;

        let mut node_queue: BinaryHeap<Pair> = BinaryHeap::with_capacity(nums.len());
        let mut nodes: Vec<Node> = nums.into_iter().enumerate()
            .map(|(i, n)| Node {
                val: n as i64,
                prev_id: if i > 0 {
                    Some(i - 1)
                } else {
                    None
                },
                next_id: if i < num_last_id {
                    Some(i + 1)
                } else {
                    None
                },
                is_merged: false
            })
            .collect();

        let mut decrease_count = 0;
        let mut op_count = 0;

        for id in 0 .. nodes.len() - 1 {
            decrease_count += (nodes[id].val > nodes[id + 1].val) as i32;

            node_queue.push(Pair {
                left_id: id,
                right_id: id + 1,
                cost: nodes[id].val + nodes[id + 1].val
            });
        }

        while let Some(node_pair) = node_queue.pop() && decrease_count > 0 {
            let (left_id, right_id, cost) 
                = (node_pair.left_id, node_pair.right_id, node_pair.cost);
            
            decrease_count -= (nodes[left_id].val > nodes[right_id].val) as i32;
            op_count += 1;

            if let Some(prev_id) = nodes[left_id].prev_id {
                decrease_count -= (nodes[prev_id].val > nodes[left_id].val) as i32;
            }

            if let Some(next_id) = nodes[right_id].next_id {
                decrease_count -= (nodes[right_id].val > nodes[next_id].val) as i32;
            }

            nodes[left_id].val = cost;
            nodes[left_id].next_id = nodes[right_id].next_id;
            nodes[right_id].is_merged = true;

            if let Some(next_id) = nodes[right_id].next_id {
                nodes[next_id].prev_id = Some(left_id);
            }

            if let Some(prev_id) = nodes[left_id].prev_id {
                decrease_count += (nodes[prev_id].val > nodes[left_id].val) as i32;

                node_queue.push(Pair {
                    left_id: prev_id,
                    right_id: left_id,
                    cost: nodes[prev_id].val + nodes[left_id].val,
                });
            }

            if let Some(next_id) = nodes[left_id].next_id {
                decrease_count += (nodes[left_id].val > nodes[next_id].val) as i32;

                node_queue.push(Pair {
                    left_id,
                    right_id: next_id,
                    cost: nodes[left_id].val + nodes[next_id].val,
                });
            }

            while let Some(next_node) = node_queue.peek() 
            && (nodes[next_node.left_id].is_merged 
            || nodes[next_node.right_id].is_merged 
            || nodes[next_node.left_id].val + nodes[next_node.right_id].val 
                != next_node.cost
            ) {
                node_queue.pop();
            }
        }

        op_count
    }
}