impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let sub_len = k as usize;
        let mut result = i64::MIN;
        let mut sub_sums: Vec<i64> = vec![i64::MAX / 2; sub_len];
        let mut prefix_sum: i64 = 0;
        
        sub_sums[sub_len - 1] = 0;

        for id in 0 .. nums.len() {
            let remainder_id = id % sub_len;

            prefix_sum += nums[id] as i64;
            result = result.max(prefix_sum - sub_sums[remainder_id]);
            sub_sums[remainder_id] = sub_sums[remainder_id].min(prefix_sum);
        }

        result
    }
}