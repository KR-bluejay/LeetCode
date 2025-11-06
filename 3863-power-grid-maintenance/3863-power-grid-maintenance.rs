use std::collections::BTreeSet;

impl Solution {
    fn precompute(
        station_id: usize,
        station_connect: &Vec<Vec<usize>>,
        station_visit: &mut Vec<bool>,
        station_grid: &mut BTreeSet<usize>,
    ) {
        if station_visit[station_id] {
            return;
        }
        station_visit[station_id] = true;
        station_grid.insert(station_id);

        for &pair_id in station_connect[station_id].iter() {
            if station_visit[pair_id] {
                continue;
            }

            Self::precompute(pair_id, station_connect, station_visit, station_grid);
        }
    }
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
        let mut station_grid: Vec<usize> = vec![0; station_count];
        let mut station_visit: Vec<bool> = vec![false; station_count];
        let mut online: Vec<bool> = vec![true; station_count];

        for id in 1 .. station_count {
            let grid_id = grids.len();
            let mut grid = BTreeSet::new();

            Self::precompute(id, &station_connect, &mut station_visit, &mut grid);

            for &target_id in grid.iter() {
                station_grid[target_id] = grid_id;
            }

            grids.push(grid.clone());
        }

        for query in queries.iter() {
            let query_type = query[0];
            let query_target = query[1] as usize;

            if query_type == 2 {
                online[query_target] = false;
                grids[station_grid[query_target]].remove(&query_target);

                continue;
            }

            if online[query_target] {
                results.push(query_target as i32);
            } else if let Some(&id) = grids[station_grid[query_target]].iter()
                .filter(|a_id| online[**a_id])
                .next() {
                results.push(id as i32);
            } else {
                results.push(-1);
            }
        }

        results
    }
}