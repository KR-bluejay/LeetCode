impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut prev_color = 0;
        let mut prev_max = 0;
        let mut prev_sum = 0;

        let mut total_time = 0;

        for (&color, time) in colors.as_bytes().into_iter().zip(needed_time.into_iter()) {
            if color == prev_color {
                prev_sum += time;
                prev_max = prev_max.max(time);

                continue;
            }

            total_time += prev_sum - prev_max;
            prev_sum = time;
            prev_max = time;
            prev_color = color;
        }

        if prev_sum > 0 {
            total_time += prev_sum - prev_max;
        }
        total_time 
    }
}