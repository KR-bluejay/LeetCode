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
        
        let mut decrease_count = 0;
        let mut op_count = 0;

        let mut nodes: Vec<Node> = Vec::with_capacity(nums.len());
        let mut node_queue: BinaryHeap<Pair> = BinaryHeap::with_capacity(nums.len() - 1);

        for id in 0 .. nums.len() {
            let num = nums[id] as i64;

            nodes.push(Node {
                val: num,
                prev_id: id.checked_sub(1),
                next_id: (id + 1 < nums.len()).then_some(id + 1),
                is_merged: false
            });

            if id + 1 < nums.len() {
                let next_num = nums[id + 1] as i64;
                let cost = num + next_num;

                decrease_count += (num > next_num) as i32;
    
                node_queue.push(Pair {
                    left_id: id,
                    right_id: id + 1,
                    cost,
                });
            }
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
            && decrease_count > 0
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