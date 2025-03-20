use std::cmp::max;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut left: usize = 0;
        let mut cur_sum = 0;

        for i in 0 .. k as usize {
            cur_sum += nums[i];
        }
        let mut max_sum = cur_sum;

        for right in k as usize .. nums.len() {
            cur_sum += nums[right];
            cur_sum -= nums[left];

            left += 1;

            max_sum = max(cur_sum, max_sum);
        }


        let max_avg: f64 = max_sum as f64 / k as f64;


        ((max_avg * 10f64.powi(5)).round() / 10f64.powi(5))
    }
}