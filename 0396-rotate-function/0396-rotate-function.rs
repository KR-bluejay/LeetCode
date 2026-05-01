impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let num_sum: i32 = nums.iter().sum();

        let mut fk = nums.iter().enumerate()
            .fold(0, |acc, (i, &n)| acc + i as i32 * n);
        let mut result = fk;

        for i in 1 .. nums.len() {
            fk = fk + num_sum - nums.len() as i32 * nums[nums.len() - i];

            result = result.max(fk);
        }

        result
    }
}