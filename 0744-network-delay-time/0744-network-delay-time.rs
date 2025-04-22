use std::collections::{ BinaryHeap, HashMap };
use std::cmp::Reverse;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut node_map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        let mut node_cost: Vec<i32> = vec![i32::MAX; (n + 1) as usize];
        let mut node_queue: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::with_capacity(n as usize);
        let mut total_min_cost: i32 = 0;

        // node_cost[k as usize] = 0;

        for i in 0 .. times.len() {
            let src_id = times[i][0];
            let dest_id = times[i][1];
            let cost = times[i][2];

            node_map.entry(src_id).or_insert(Vec::new())
                .push((dest_id, cost));
        }


        node_cost[0] = 0;
        // node_cost[k as usize] = 0;
        node_queue.push(Reverse((k, 0)));


        while !node_queue.is_empty() {
            let Reverse((src_id, cost)) = node_queue.pop().unwrap();

            if node_cost[src_id as usize] < cost {
                continue;
            }

            node_cost[src_id as usize] = cost;


            if let Some(adj_map) = node_map.get(&src_id) {
                for (adj_id, adj_cost) in adj_map.iter() {
                    let adj_id = *adj_id as usize;
                    if node_cost[adj_id] <= cost + adj_cost {
                        continue;
                    }
                    node_cost[adj_id] = cost + adj_cost;
                    node_queue.push(Reverse((adj_id as i32, cost + adj_cost)));
                }
            }
        }

        for &item in node_cost.iter() {
            if item == i32::MAX {
                return -1;
            }
            total_min_cost = total_min_cost.max(item);
        }
        total_min_cost
    }
}