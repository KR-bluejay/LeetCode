use std::cmp::max;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut left: usize = 0;
        let mut cur_sum = 0;
        let mut max_sum = -1000000; 

        for right in 0 .. nums.len() {
            cur_sum += &nums[right as usize];

            if right - left > k as usize - 1 {
                cur_sum -= &nums[left as usize];
                left += 1;
            }

            if right - left == k as usize - 1 {
                println!("{cur_sum}");
                max_sum = max(cur_sum, max_sum)
            }
        }


        let max_avg: f64 = max_sum as f64 / k as f64;


        ((max_avg * 10f64.powi(5)).round() / 10f64.powi(5))
    }
}