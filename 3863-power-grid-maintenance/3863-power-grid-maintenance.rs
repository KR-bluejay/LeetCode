use std::collections::{VecDeque};

impl Solution {
    pub fn process_queries(
        c: i32, 
        connections: Vec<Vec<i32>>, 
        queries: Vec<Vec<i32>>
    ) -> Vec<i32> {
        let station_count = c as usize + 1;
        let mut station_connect: Vec<Vec<usize>> 
            = vec![Vec::with_capacity(station_count / 2); station_count];
        let mut results: Vec<i32> = Vec::with_capacity(station_count);

        for connection in connections.iter() {
            station_connect[connection[0] as usize].push(connection[1] as usize);
            station_connect[connection[1] as usize].push(connection[0] as usize);
        }

        let mut grids: Vec<Vec<usize>> = Vec::with_capacity(station_count);
        let mut station_grid: Vec<usize> = vec![usize::MAX; station_count];
        let mut online: Vec<bool> = vec![true; station_count];
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(station_count);
        let mut grid = Vec::with_capacity(station_count);

        for id in 1 .. station_count {
            if station_grid[id] != usize::MAX {
                continue;
            }

            let grid_id = grids.len();

            queue.push_back(id);

            while let Some(station_id) = queue.pop_front() {
                grid.push(station_id);
                station_grid[station_id] = grid_id;

                for &pair_id in station_connect[station_id].iter() {
                    if station_grid[pair_id] != usize::MAX {
                        continue;
                    }
                    station_grid[pair_id] = grid_id;
                    queue.push_back(pair_id);
                }
            }

            grid.sort_unstable();
            grids.push(grid.clone());
            queue.clear();
            grid.clear();
        }

        let mut grid_online_index: Vec<usize> = vec![0; grids.len()];

        for query in queries.iter() {
            let query_type = query[0];
            let query_target = query[1] as usize;
            let prev_len = results.len();

            if query_type == 2 {
                online[query_target] = false;

                continue;
            }

            if online[query_target] {
                results.push(query_target as i32);

                continue;
            }

            let grid_id = station_grid[query_target];
            let grid_nodes = &grids[grid_id];
            let mut cur_id = grid_online_index[grid_id];

            while cur_id < grid_nodes.len() {
                let candidate_id = grid_nodes[cur_id];

                if online[candidate_id] {
                    results.push(candidate_id as i32);
                    grid_online_index[grid_id] = cur_id;
                    break;
                } else {
                    cur_id += 1;
                }
            }

            if prev_len == results.len() {
                results.push(-1);
                grid_online_index[grid_id] = grid_nodes.len();
            }
        }

        results
    }
}