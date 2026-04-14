impl Solution {
    pub fn minimum_total_distance(
        mut robots: Vec<i32>, 
        mut factories: Vec<Vec<i32>>
    ) -> i64 {
        robots.sort_unstable();
        factories.sort_unstable();

        let mut factory_pos = Vec::with_capacity(robots.len());


        for factory in factories.into_iter() {
            let (pos, limit) = (factory[0], factory[1]);

            for _ in 0 .. limit {
                factory_pos.push(pos);
            }
        }

        let mut cache = vec![vec![-1; factory_pos.len()]; robots.len()];

        Self::calc(&mut cache, &robots, 0, &factory_pos, 0);

        cache[0][0]
    }
    fn calc(
        cache: &mut Vec<Vec<i64>>,
        robots: &Vec<i32>,
        robot_id: usize,
        factories: &Vec<i32>,
        factory_id: usize,
    ) -> i64 {
        if robot_id == robots.len() {
            return 0;
        }

        if factory_id == factories.len() {
            return i64::MAX / 2;
        }

        if cache[robot_id][factory_id] != -1 {
            return cache[robot_id][factory_id];
        }

        let cur_dist = (robots[robot_id] - factories[factory_id])
            .abs() as i64;
        
        let assign = cur_dist + Self::calc(
            cache,
            robots, 
            robot_id + 1, 
            factories, 
            factory_id + 1
        );
        let skip = Self::calc(
            cache,
            robots,
            robot_id,
            factories,
            factory_id + 1,
        );


        cache[robot_id][factory_id] = assign.min(skip);
        cache[robot_id][factory_id]
    }

}