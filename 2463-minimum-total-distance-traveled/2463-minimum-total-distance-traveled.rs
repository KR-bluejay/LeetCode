use std::iter::repeat_n;

impl Solution {
    pub fn minimum_total_distance(mut robots: Vec<i32>, mut factories: Vec<Vec<i32>>) -> i64 {
        robots.sort_unstable();
        factories.sort_unstable();

        let mut factory_positions = Vec::with_capacity(robots.len());

        for factory in factories.into_iter() {
            let (position, limit) = (factory[0], factory[1] as usize);

            factory_positions.extend(repeat_n(position, limit));
        }

        let mut min_dist_cache = vec![vec![i64::MAX >> 2; factory_positions.len() + 1]; robots.len() + 1];

        for factory_id in 0 ..= factory_positions.len() {
            min_dist_cache[0][factory_id] = 0;
        }

        for robot_id in 1 ..= robots.len() {
            for factory_id in robot_id ..= factory_positions.len() {
                let dist = (robots[robot_id - 1] - factory_positions[factory_id - 1]).abs() as i64;

                let assign_cost = min_dist_cache[robot_id - 1][factory_id - 1] + dist;
                let skip_cost = min_dist_cache[robot_id][factory_id - 1];

                min_dist_cache[robot_id][factory_id] = assign_cost.min(skip_cost);
            }
        }


        *min_dist_cache.last().unwrap().last().unwrap()
    }
}