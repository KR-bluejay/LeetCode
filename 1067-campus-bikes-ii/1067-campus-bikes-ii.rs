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

        let all_count = (1 << bikes.len()) - 1;
        let mut remaining_count = (!bit_mask) & all_count;

        while remaining_count != 0 {
            let bike_id = remaining_count.trailing_zeros() as usize;
            let cur_dist = ongoing_dist + (workers[0][0] - bikes[bike_id][0]).abs() 
            + (workers[0][1] - bikes[bike_id][1]).abs();

            remaining_count &= (remaining_count - 1);

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