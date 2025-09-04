impl Solution {
    fn dyn_assign(
        workers: &[Vec<i32>],
        bikes: &Vec<Vec<i32>>,
        bit_mask: usize, 
        mut ongoing_dist: i32,
        short_dist: &mut i32
    ) {
        if ongoing_dist >= *short_dist {
            return;
        }

        if workers.len() == 0 {
            *short_dist = ongoing_dist;

            return;
        }

        for bike_id in 0 .. bikes.len() {
            let is_used_bike = (bit_mask & (1 << bike_id)) != 0;
            
            if is_used_bike {
                continue;
            }

            let cur_dist = ongoing_dist + (workers[0][0] - bikes[bike_id][0]).abs() 
            + (workers[0][1] - bikes[bike_id][1]).abs();


            Self::dyn_assign(
                &workers[1 ..], 
                bikes, 
                bit_mask ^ (1 << bike_id), 
                cur_dist, 
                short_dist
            );
        }
    }
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        let mut short_dist = i32::MAX;

        Self::dyn_assign(&workers[0 ..], &bikes, 0, 0, &mut short_dist);

        short_dist
    }
}