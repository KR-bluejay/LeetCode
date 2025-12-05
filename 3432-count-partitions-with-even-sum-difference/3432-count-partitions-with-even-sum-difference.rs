impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0;
        let mut right_sum: i32 = nums.iter().sum();
        let mut result = 0;

        for &num in nums.iter().take(nums.len() - 1) {
            left_sum += num;
            right_sum -= num;

            result += ((right_sum - left_sum) % 2 == 0) as i32;
        }
        result
    }
}