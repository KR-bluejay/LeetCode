use std::collections::BinaryHeap;
use std::cmp::{ Ordering, Reverse };

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let last_node_id = n as usize - 1;

        let mut node_state: Vec<i32> = vec![i32::MAX; n as usize];
        let mut node_heap: BinaryHeap<(Reverse<i32>, usize)> 
            = BinaryHeap::with_capacity(n as usize);

        let mut graph: Vec<Vec<(usize, i32)>> 
            = vec![Vec::with_capacity(2); n as usize];


        for edge in edges.into_iter() {
            let (u, v, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);

            graph[u].push((v, cost));
            graph[v].push((u, cost * 2));
        }

        node_heap.push((Reverse(0), 0));
        node_state[0] = 0;


        while let Some(node) = node_heap.pop() {
            let (id, cost) = (node.1, node.0.0);

            if node_state[id] < cost {
                continue;
            }

            if id == last_node_id {
                return cost;
            }

            for next_node in graph[id].iter() {
                let next_id = next_node.0;
                let next_cost = next_node.1 + cost;

                if node_state[next_id] <= next_cost {
                    continue;
                }

                node_state[next_id] = next_cost;
                node_heap.push((Reverse(next_cost), next_id));
            }
        }

        -1
    }
}