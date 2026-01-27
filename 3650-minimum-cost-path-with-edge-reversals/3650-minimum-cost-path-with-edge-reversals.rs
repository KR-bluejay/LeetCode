use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    id: usize,
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id).reverse()
            // .then(self.cost.cmp(&other.cost).reverse())
            .then(self.cost.cmp(&other.cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut node_state: Vec<i32> = vec![i32::MAX; n as usize];
        let mut node_heap: BinaryHeap<Node> = BinaryHeap::with_capacity(n as usize);

        let mut graph: Vec<Vec<(usize, i32)>> 
            = vec![Vec::with_capacity(2); n as usize];
        let mut rev_graph: Vec<Vec<(usize, i32)>> 
            = vec![Vec::with_capacity(2); n as usize];

        for edge in edges.into_iter() {
            let (u, v, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);

            graph[u].push((v, cost));
            rev_graph[v].push((u, cost * 2));
        }

        node_heap.push(Node {
            id: 0,
            cost: 0,
        });
        node_state[0] = 0;

        for next_node in rev_graph[0].iter() {
            let next_id = next_node.0;
            let next_cost = next_node.1;
            
            node_heap.push(Node {
                id: next_id,
                cost: next_cost,
            });
            node_state[next_id] = next_cost;
        }

        while let Some(node) = node_heap.pop() {
            let Node { id, cost } = node;

            if node_state[id] < cost {
                continue;
            }

            for next_node in graph[id].iter() {
                let next_id = next_node.0;
                let next_cost = next_node.1 + cost;

                if node_state[next_id] <= next_cost {
                    continue;
                }

                node_state[next_id] = next_cost;
                node_heap.push(Node {
                    id: next_id,
                    cost: next_cost,
                });
            }

            for next_node in rev_graph[id].iter() {
                let next_id = next_node.0;
                let next_cost = next_node.1 + cost;

                if node_state[next_id] <= next_cost || id == next_id {
                    continue;
                }

                node_state[next_id] = next_cost;
                
                node_heap.push(Node {
                    id: next_id,
                    cost: next_cost,
                });
            }
        }
        
        if node_state[n as usize - 1] == i32::MAX {
            -1
        } else {
            node_state[n as usize - 1] 
        }
    }
}