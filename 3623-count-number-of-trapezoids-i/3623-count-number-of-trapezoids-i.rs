impl Solution {
    pub fn count_trapezoids(mut points: Vec<Vec<i32>>) -> i32 {
        const MODULO: usize = 1_000_000_007;

        let mut point_sum: usize = 0;
        let mut result: usize = 0;

        points.sort_by(|lhs, rhs| lhs[1].cmp(&rhs[1]));

        let mut left_id = 0;

        while left_id < points.len() {
            let mut count = 1; 
            let mut right_id = left_id + 1;

            while right_id < points.len() && points[right_id][1] == points[left_id][1] {
                count += 1;
                right_id += 1;
            }

            result += count * (count - 1) / 2 * point_sum;
            result %= MODULO;

            point_sum += count * (count - 1) / 2;

            left_id = right_id;
        }

        result as i32
    }
}