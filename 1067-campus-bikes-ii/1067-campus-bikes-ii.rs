impl Solution {
    fn dyn_man_dist(
        workers: &[Vec<i32>], 
        bikes: &Vec<Vec<i32>>, 
        bikes_use: &mut Vec<bool>,
        mut min_dist: i32, 
        mut cur_dist: i32
    ) -> i32 {
        if cur_dist > min_dist {
            return i32::MAX;
        }

        if workers.len() == 0 {
            return cur_dist;
        }


        for i in 0 .. bikes.len() {
            if bikes_use[i] {
                continue;
            }
            bikes_use[i] = true;
            let work_bike_dist = (workers[0][0] - bikes[i][0]).abs() + (workers[0][1] - bikes[i][1]).abs();
            let new_dist = Self::dyn_man_dist(&workers[1 ..], bikes, bikes_use, min_dist, cur_dist + work_bike_dist);
            min_dist = min_dist.min(new_dist);
            bikes_use[i] = false;
        }


        min_dist
    }
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        let mut min_dist: i32 = i32::MAX;
        let mut bikes_use: Vec<bool> = vec![false; bikes.len()];

        Self::dyn_man_dist(&workers[0 ..], &bikes, &mut bikes_use, i32::MAX, 0)
    }
}