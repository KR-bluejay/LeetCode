
impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let sub_len = k as usize;
        let mut prefix: Vec<i64> = Vec::with_capacity(nums.len() + 1);
        prefix.push(0);

        for id in 1 ..= nums.len() {
            prefix.push(prefix[id - 1] + nums[id - 1] as i64);
        }

        let mut min_prefix: Vec<i64> = vec![i64::MAX; sub_len];
        let mut result = i64::MIN;

        for id in 0 ..= nums.len() {
            let prefix_id = id % sub_len;

            if min_prefix[prefix_id] != i64::MAX {
                result = result.max(prefix[id] - min_prefix[prefix_id]);
            }

            min_prefix[prefix_id] = min_prefix[prefix_id].min(prefix[id]);
        }

        result
    }
}