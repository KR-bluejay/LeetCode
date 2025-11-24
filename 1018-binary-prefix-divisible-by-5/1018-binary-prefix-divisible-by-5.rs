impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut results: Vec<bool> = Vec::with_capacity(nums.len());
        let mut prefix_num: usize = 0;

        for (num_id, &num_val) in nums.iter().enumerate() {
            prefix_num <<= 1;
            prefix_num += num_val as usize;
            prefix_num %= 5;

            results.push(prefix_num % 5 == 0);
        }

        results
    }
}