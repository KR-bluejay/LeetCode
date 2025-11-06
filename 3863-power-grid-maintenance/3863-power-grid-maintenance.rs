use std::collections::{BTreeSet, VecDeque};

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

        let mut grids: Vec<BTreeSet<usize>> = Vec::with_capacity(station_count);
        let mut station_grid: Vec<usize> = vec![usize::MAX; station_count];
        let mut online: Vec<bool> = vec![true; station_count];
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(station_count);

        for id in 1 .. station_count {
            if station_grid[id] != usize::MAX {
                continue;
            }

            let grid_id = grids.len();
            let mut grid = BTreeSet::new();

            queue.push_back(id);

            while let Some(station_id) = queue.pop_front() {
                grid.insert(station_id);
                station_grid[station_id] = grid_id;

                for &pair_id in station_connect[station_id].iter() {
                    if station_grid[pair_id] != usize::MAX {
                        continue;
                    }
                    station_grid[pair_id] = grid_id;
                    queue.push_back(pair_id);
                }
            }

            grids.push(grid.clone());
            queue.clear();
        }

        for query in queries.iter() {
            let query_type = query[0];
            let query_target = query[1] as usize;

            if query_type == 2 {
                online[query_target] = false;
                // grids[station_grid[query_target]].remove(&query_target);

                continue;
            }
            let prev_len = results.len();

            if online[query_target] {
                results.push(query_target as i32);

                continue;
            } else {
                while let Some(&a_id) = grids[station_grid[query_target]].iter().next() {
                    if online[a_id] {
                        results.push(a_id as i32);
                        
                        break;
                    } else {
                        grids[station_grid[a_id]].remove(&a_id);
                    }
                }
            }
            if prev_len == results.len() {
                results.push(-1);
            }
        }

        results
    }
}