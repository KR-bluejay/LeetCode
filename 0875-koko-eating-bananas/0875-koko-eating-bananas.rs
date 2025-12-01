impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left_speed = 1;
        let mut right_speed = *piles.iter().max().unwrap();

        while left_speed < right_speed {
            let mid_speed = left_speed + (right_speed - left_speed) / 2;
            let mut total_speed = 0;


            for &pile in piles.iter() {
                total_speed += (pile + mid_speed - 1) / mid_speed;

                if h < total_speed {
                    break;
                }
            }

            if h < total_speed {
                left_speed = mid_speed + 1;
            } else {
                right_speed = mid_speed;
            }
        }

        left_speed
    }
}