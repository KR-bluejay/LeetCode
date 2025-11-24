impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut results: Vec<bool> = Vec::with_capacity(nums.len());
        let mut prefix_num = 0;

        for num_val in nums.into_iter() {
            prefix_num = (prefix_num << 1 | num_val) % 5;

            results.push(prefix_num == 0);
        }

        results
    }
}