impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }

        nums.sort_unstable();

        let mut k = k as usize;
        let mut result = i32::MAX;

        for id in 0 .. (nums.len() - k + 1) {
            result = result.min(nums[id + k - 1] - nums[id]);
        }

        result
    }
}