impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut left_id = 0;
        let mut max_len = 0;

        let k = k as i64;

        for right_id in 0 .. nums.len() {
            if nums[left_id] as i64 * k < nums[right_id] as i64 {
                left_id += 1;
            }
            max_len = max_len.max(right_id - left_id + 1);
        }

        (nums.len() - max_len) as i32
    }
}